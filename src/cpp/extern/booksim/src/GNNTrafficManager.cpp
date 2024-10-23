// Copyright (c) 2009-2013, Tor M. Aamodt, Dongdong Li, Ali Bakhoda
// The University of British Columbia
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// Redistributions of source code must retain the above copyright notice, this
// list of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or
// other materials provided with the distribution.
// Neither the name of The University of British Columbia nor the names of its
// contributors may be used to endorse or promote products derived from this
// software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#include <sstream>
#include <fstream>
#include <limits> 

#include "GNNTrafficManager.hpp"
#include "Interconnect.hpp"

GNNTrafficManager::GNNTrafficManager( const Configuration &config, const vector<Network *> &net,
                                     booksim2::Interconnect* icnt)
:TrafficManager(config, net, icnt)
{
  // The total simulations equal to number of kernels
  _total_sims = 0;
  
  _input_queue.resize(_subnets);
  for ( unsigned subnet = 0; subnet < _subnets; ++subnet) {
    _input_queue[subnet].resize(_nodes);
    for ( unsigned node = 0; node < _nodes; ++node ) {
      _input_queue[subnet][node].resize(_classes);
    }
  }
  flit_size = config.GetInt("flit_size");
}

GNNTrafficManager::~GNNTrafficManager()
{
}

void GNNTrafficManager::Init()
{
  _time = 0;
  _sim_state = running;
  _ClearStats( );
  
}

void GNNTrafficManager::_RetireFlit( Flit *f, unsigned dest )
{
  _deadlock_timer = 0;
  
  assert(_total_in_flight_flits[f->cl].count(f->id) > 0);
  _total_in_flight_flits[f->cl].erase(f->id);
  
  if(f->record) {
    assert(_measured_in_flight_flits[f->cl].count(f->id) > 0);
    _measured_in_flight_flits[f->cl].erase(f->id);
  }
  
  if ( f->watch ) {
    *(icnt->gWatchOut) << icnt->get_cycle() << " | "
    << "node" << dest << " | "
    << "Retiring flit " << f->id
    << " (packet " << f->pid
    << ", src = " << f->src
    << ", dest = " << f->dest
    << ", hops = " << f->hops
    << ", flat = " << f->atime - f->itime
    << ")." << endl;
  }
  
  if ( f->head && ( f->dest != dest ) ) {
    ostringstream err;
    err << "Flit " << f->id << " arrived at incorrect output " << dest;
    Error( err.str( ) );
  }
  
  if((_slowest_flit[f->cl] < 0) ||
     (_flat_stats[f->cl]->Max() < (f->atime - f->itime)))
    _slowest_flit[f->cl] = f->id;
  
  _flat_stats[f->cl]->AddSample( f->atime - f->itime);
  if(_pair_stats){
    _pair_flat[f->cl][f->src*_nodes+dest]->AddSample( f->atime - f->itime );
  }
  
  if ( f->tail ) {
    Flit * head;
    if(f->head) {
      head = f;
    } else {
      map<unsigned long long, Flit *>::iterator iter = _retired_packets[f->cl].find(f->pid);
      assert(iter != _retired_packets[f->cl].end());
      head = iter->second;
      _retired_packets[f->cl].erase(iter);
      assert(head->head);
      assert(f->pid == head->pid);
    }
    if ( f->watch ) {
      *(icnt->gWatchOut) << icnt->get_cycle() << " | "
      << "node" << dest << " | "
      << "Retiring packet " << f->pid
      << " (plat = " << f->atime - head->ctime
      << ", nlat = " << f->atime - head->itime
      << ", frag = " << (f->atime - head->atime) - (f->id - head->id) // NB: In the spirit of solving problems using ugly hacks, we compute the packet length by taking advantage of the fact that the IDs of flits within a packet are contiguous.
      << ", src = " << head->src
      << ", dest = " << head->dest
      << ")." << endl;
    }
   
// GPGPUSim: Memory will handle reply, do not need this
#if 0
    //code the source of request, look carefully, its tricky ;)
    if (f->type == Flit::READ_REQUEST || f->type == Flit::WRITE_REQUEST) {
      PacketReplyInfo* rinfo = PacketReplyInfo::New();
      rinfo->source = f->src;
      rinfo->time = f->atime;
      rinfo->record = f->record;
      rinfo->type = f->type;
      _repliesPending[dest].push_back(rinfo);
    } else {
      if(f->type == Flit::READ_REPLY || f->type == Flit::WRITE_REPLY  ){
        _requestsOutstanding[dest]--;
      } else if(f->type == Flit::ANY_TYPE) {
        _requestsOutstanding[f->src]--;
      }
      
    }
#endif

    if(f->type == Flit::READ_REPLY || f->type == Flit::WRITE_REPLY  ){
      _requestsOutstanding[dest]--;
    } else if(f->type == Flit::ANY_TYPE) {
      ostringstream err;
      err << "Flit " << f->id << " cannot be ANY_TYPE" ;
      Error( err.str( ) );
    }
    
    // Only record statistics once per packet (at tail)
    // and based on the simulation state
    if ( ( _sim_state == warming_up ) || f->record ) {
      
      _hop_stats[f->cl]->AddSample( f->hops );
      
      if((_slowest_packet[f->cl] < 0) ||
         (_plat_stats[f->cl]->Max() < (f->atime - head->itime)))
        _slowest_packet[f->cl] = f->pid;
      _plat_stats[f->cl]->AddSample( f->atime - head->ctime);
      _nlat_stats[f->cl]->AddSample( f->atime - head->itime);
      _frag_stats[f->cl]->AddSample( (f->atime - head->atime) - (f->id - head->id) );
      
      if(_pair_stats){
        _pair_plat[f->cl][f->src*_nodes+dest]->AddSample( f->atime - head->ctime );
        _pair_nlat[f->cl][f->src*_nodes+dest]->AddSample( f->atime - head->itime );
      }
    }
    
    if(f != head) {
      head->Free();
    }
    
  }
  
  if(f->head && !f->tail) {
    _retired_packets[f->cl].insert(make_pair(f->pid, f));
  } else {
    f->Free();
  }
}
int  GNNTrafficManager::_IssuePacket( int source, int cl )
{
  return 0;
}

//TODO: Remove stype?
//void GNNTrafficManager::_GeneratePacket(int source, int stype, int cl, int time, int subnet, int packet_size, const Flit::FlitType& packet_type, void* const data, int dest)
void GNNTrafficManager::_GeneratePacket(void* packet,
                                        uint64_t addr, int bytes, booksim2::Interconnect::Type type,
                                        int header_size, uint32_t subnet, int cl, int time,
                                        unsigned src, unsigned dst) {
  //  Flit::FlitType packet_type = Flit::ANY_TYPE;
  unsigned long long pid = _cur_pid++;
  assert(_cur_pid > 0);
  unsigned source = src;
  unsigned packet_destination = dst;
  bool record = false;
  bool watch = (icnt->gWatchOut) && (_packets_to_watch.count(pid) > 0);
  Flit::FlitType packet_type;
  if (type == booksim2::Interconnect::Type::READ) {
    packet_type = Flit::READ_REQUEST;
  } else if (type == booksim2::Interconnect::Type::WRITE) {
    packet_type = Flit::WRITE_REQUEST;
  } else if (type == booksim2::Interconnect::Type::READ_REPLY) {
    packet_type = Flit::READ_REPLY;
  } else if (type == booksim2::Interconnect::Type::WRITE_REPLY) {
    // FIX: Do we need write reply (ACK)?
    packet_type = Flit::WRITE_REPLY;
    assert(false && "No write reply currently");
  } else {
    packet_type = Flit::ANY_TYPE;
    cout << "Packet type is undefined!" << endl;
    assert(0 && "Type is undefined");
  }
  int pkt_size = bytes + header_size;
  int num_flits = (pkt_size / flit_size) + ((pkt_size % flit_size) ? 1 : 0);

  if ((packet_destination <0) || (packet_destination >= _nodes)) {
    ostringstream err;
    err << "Incorrect packet destination " << packet_destination
    << " for stype " << packet_type;
    Error( err.str( ) );
  }
  
  if ( ( _sim_state == running ) ||
      ( ( _sim_state == draining ) && ( time < _drain_time ) ) ) {
    record = _measure_stats[cl];
  }
  
  int subnetwork = subnet;
  //                ((packet_type == Flit::ANY_TYPE) ?
  //                    RandomInt(_subnets-1) :
  //                    _subnet[packet_type]);
  
  if ( watch ) {
    *(icnt->gWatchOut) << icnt->get_cycle() << " | "
    << "node" << source << " | "
    << "Enqueuing packet " << pid
    << " at time " << time
    << "." << endl;
  }
  
  //NetworkRequest *_net_req = new NetworkRequest(net_req);
  for ( int i = 0; i < num_flits; ++i ) {
    bool is_tail = i == num_flits - 1;
    Flit * f  = Flit::New();
    f->id     = _cur_id++;
    assert(_cur_id);
    f->pid    = pid;
    f->watch  = watch | ((icnt->gWatchOut) && (_flits_to_watch.count(f->id) > 0));
    f->subnetwork = subnetwork;
    f->src    = source;
    f->ctime  = time;
    f->record = record;
    f->cl     = cl;
    f->payload_size = bytes;
    if (is_tail && (pkt_size % flit_size != 0)) {
      f->size = pkt_size % flit_size;
    } else {
      f->size = flit_size;
    }
    // store data only for the tail flit
    f->data = is_tail ? packet : nullptr;
    
    _total_in_flight_flits[f->cl].insert(make_pair(f->id, f));
    if(record) {
      _measured_in_flight_flits[f->cl].insert(make_pair(f->id, f));
    }
    
    if(icnt->gTrace){
      cout<<"New Flit "<<f->src<<endl;
    }
    f->type = packet_type;
    f->head = i == 0;
    if (f->head) { // Head flit
      //packets are only generated to nodes smaller or equal to limit
      f->dest = packet_destination;
    } else {
      f->dest = -1;
    }
    switch( _pri_type ) {
      case class_based:
        f->pri = _class_priority[cl];
        assert(f->pri >= 0);
        break;
      case age_based:
        f->pri = numeric_limits<int>::max() - time;
        assert(f->pri >= 0);
        break;
      case sequence_based:
        f->pri = numeric_limits<int>::max() - _packet_seq_no[source];
        assert(f->pri >= 0);
        break;
      default:
        f->pri = 0;
    }
    f->tail = is_tail;
    
    f->vc  = -1;
    
    if ( f->watch ) {
      *(icnt->gWatchOut) << icnt->get_cycle() << " | "
      << "node" << source << " | "
      << "Enqueuing flit " << f->id
      << " (packet " << f->pid
      << ") at time " << time
      << "." << endl;
    }
    
    _input_queue[subnet][source][cl].push_back( f );
  }
}

void GNNTrafficManager::_Step()
{
  bool flits_in_flight = false;
  for(unsigned c = 0; c < _classes; ++c) {
    flits_in_flight |= !_total_in_flight_flits[c].empty();
  }
  if(flits_in_flight && (_deadlock_timer++ >= _deadlock_warn_timeout)){
    _deadlock_timer = 0;
    cout << "WARNING: Possible network deadlock.\n";
  }
  
  vector<map<int, Flit *> > flits(_subnets);
  
  for ( unsigned subnet = 0; subnet < _subnets; ++subnet ) {
    for ( unsigned n = 0; n < _nodes; ++n ) {
      Flit * const f = _net[subnet]->ReadFlit( n );
      if ( f ) {
        if(f->watch) {
          *(icnt->gWatchOut) << icnt->get_cycle() << " | "
          << "node" << n << " | "
          << "Ejecting flit " << f->id
          << " (packet " << f->pid << ")"
          << " from VC " << f->vc
          << "." << endl;
        }
        icnt->WriteOutBuffer(subnet, n, f);
      }
      
      icnt->Transfer2BoundaryBuffer(subnet, n);
      Flit* const ejected_flit = icnt->GetEjectedFlit(subnet, n);
      if (ejected_flit) {
        if(ejected_flit->head)
          assert(ejected_flit->dest == n);
        if(ejected_flit->watch) {
          *(icnt->gWatchOut) << icnt->get_cycle() << " | "
          << "node" << n << " | "
          << "Ejected flit " << ejected_flit->id
          << " (packet " << ejected_flit->pid
          << " VC " << ejected_flit->vc << ")"
          << "from ejection buffer." << endl;
        }
        flits[subnet].insert(make_pair(n, ejected_flit));
        if((_sim_state == warming_up) || (_sim_state == running)) {
          ++_accepted_flits[ejected_flit->cl][n];
          if(ejected_flit->tail) {
            ++_accepted_packets[ejected_flit->cl][n];
          }
        }
      }
    
      // Processing the credit From the network
      Credit * const c = _net[subnet]->ReadCredit( n );
      if ( c ) {
#ifdef TRACK_FLOWS
        for(set<int>::const_iterator iter = c->vc.begin(); iter != c->vc.end(); ++iter) {
          int const vc = *iter;
          assert(!_outstanding_classes[n][subnet][vc].empty());
          int cl = _outstanding_classes[n][subnet][vc].front();
          _outstanding_classes[n][subnet][vc].pop();
          assert(_outstanding_credits[cl][subnet][n] > 0);
          --_outstanding_credits[cl][subnet][n];
        }
#endif
        _buf_states[n][subnet]->ProcessCredit(c);
        c->Free();
      }
    }
    _net[subnet]->ReadInputs( );
  }

// GPGPUSim will generate/inject packets from interconnection interface
#if 0
  if ( !_empty_network ) {
    _Inject();
  }
#endif
  
  for(unsigned subnet = 0; subnet < _subnets; ++subnet) {
    
    for(unsigned n = 0; n < _nodes; ++n) {
      
      Flit * f = NULL;
      
      BufferState * const dest_buf = _buf_states[n][subnet];
      
      int const last_class = _last_class[n][subnet];
      
      int class_limit = _classes;
      
      if(_hold_switch_for_packet) {
        list<Flit *> const & pp = _input_queue[subnet][n][last_class];
        if(!pp.empty() && !pp.front()->head &&
           !dest_buf->IsFullFor(pp.front()->vc)) {
          f = pp.front();
          assert(f->vc == _last_vc[n][subnet][last_class]);
          
          // if we're holding the connection, we don't need to check that class
          // again in the for loop
          --class_limit;
        }
      }
      
      for(int i = 1; i <= class_limit; ++i) {
        
        int const c = (last_class + i) % _classes;
        
        list<Flit *> const & pp = _input_queue[subnet][n][c];
        
        if(pp.empty()) {
          continue;
        }
        
        Flit * const cf = pp.front();
        assert(cf);
        assert(cf->cl == c);
        
        assert(cf->subnetwork == subnet);
        
        if(f && (f->pri >= cf->pri)) {
          continue;
        }
        
        if(cf->head && cf->vc == -1) { // Find first available VC
          
          const FlitChannel * inject = _net[subnet]->GetInject(n);
          const Router * router = inject->GetSink();
          assert(router);
          OutputSet route_set;
          _rf(router, cf, -1, &route_set, true);
          set<OutputSet::sSetElement> const & os = route_set.GetSet();
          assert(os.size() == 1);
          OutputSet::sSetElement const & se = *os.begin();
          assert(se.output_port == -1);
          int vc_start = se.vc_start;
          int vc_end = se.vc_end;
          int vc_count = vc_end - vc_start + 1;
          if(_noq) {
            assert(_lookahead_routing);
            //const FlitChannel * inject = _net[subnet]->GetInject(n);
            //const Router * router = inject->GetSink();
            //assert(router);
            int in_channel = inject->GetSinkPort();
            
            // NOTE: Because the lookahead is not for injection, but for the
            // first hop, we have to temporarily set cf's VC to be non-negative
            // in order to avoid seting of an assertion in the routing function.
            cf->vc = vc_start;
            _rf(router, cf, in_channel, &cf->la_route_set, false);
            cf->vc = -1;
            
            if(cf->watch) {
              *(icnt->gWatchOut) << icnt->get_cycle() << " | "
              << "node" << n << " | "
              << "Generating lookahead routing info for flit " << cf->id
              << " (NOQ)." << endl;
            }
            set<OutputSet::sSetElement> const sl = cf->la_route_set.GetSet();
            assert(sl.size() == 1);
            int next_output = sl.begin()->output_port;
            vc_count /= router->NumOutputs();
            vc_start += next_output * vc_count;
            vc_end = vc_start + vc_count - 1;
            assert(vc_start >= se.vc_start && vc_start <= se.vc_end);
            assert(vc_end >= se.vc_start && vc_end <= se.vc_end);
            assert(vc_start <= vc_end);
          }
          if(cf->watch) {
            *(icnt->gWatchOut) << icnt->get_cycle() << " | " << FullName() << " | "
            << "Finding output VC for flit " << cf->id
            << ":" << endl;
          }
          for(int i = 1; i <= vc_count; ++i) {
            int const lvc = _last_vc[n][subnet][c];
            int const vc =
            (lvc < vc_start || lvc > vc_end) ?
            vc_start :
            (vc_start + (lvc - vc_start + i) % vc_count);
            assert((vc >= vc_start) && (vc <= vc_end));
            if(!dest_buf->IsAvailableFor(vc)) {
              if(cf->watch) {
                *(icnt->gWatchOut) << icnt->get_cycle() << " | " << FullName() << " | "
                << "  Output VC " << vc << " is busy." << endl;
              }
            } else {
              if(dest_buf->IsFullFor(vc)) {
                if(cf->watch) {
                  *(icnt->gWatchOut) << icnt->get_cycle() << " | " << FullName() << " | "
                  << "  Output VC " << vc << " is full." << endl;
                }
              } else {
                if(cf->watch) {
                  *(icnt->gWatchOut) << icnt->get_cycle() << " | " << FullName() << " | "
                  << "  Selected output VC " << vc << "." << endl;
                }
                cf->vc = vc;
                break;
              }
            }
          }
        }
        
        if(cf->vc == -1) {
          if(cf->watch) {
            *(icnt->gWatchOut) << icnt->get_cycle() << " | " << FullName() << " | "
            << "No output VC found for flit " << cf->id
            << "." << endl;
          }
        } else {
          if(dest_buf->IsFullFor(cf->vc)) {
            if(cf->watch) {
              *(icnt->gWatchOut) << icnt->get_cycle() << " | " << FullName() << " | "
              << "Selected output VC " << cf->vc
              << " is full for flit " << cf->id
              << "." << endl;
            }
          } else {
            f = cf;
          }
        }
      }
      
      if(f) {
        
        assert(f->subnetwork == subnet);
        
        int const c = f->cl;
        
        if(f->head) {
          
          if (_lookahead_routing) {
            if(!_noq) {
              const FlitChannel * inject = _net[subnet]->GetInject(n);
              const Router * router = inject->GetSink();
              assert(router);
              int in_channel = inject->GetSinkPort();
              _rf(router, f, in_channel, &f->la_route_set, false);
              if(f->watch) {
                *(icnt->gWatchOut) << icnt->get_cycle() << " | "
                << "node" << n << " | "
                << "Generating lookahead routing info for flit " << f->id
                << "." << endl;
              }
            } else if(f->watch) {
              *(icnt->gWatchOut) << icnt->get_cycle() << " | "
              << "node" << n << " | "
              << "Already generated lookahead routing info for flit " << f->id
              << " (NOQ)." << endl;
            }
          } else {
            f->la_route_set.Clear();
          }
          
          dest_buf->TakeBuffer(f->vc);
          _last_vc[n][subnet][c] = f->vc;
        }
        
        _last_class[n][subnet] = c;
        
        _input_queue[subnet][n][c].pop_front();
        
#ifdef TRACK_FLOWS
        ++_outstanding_credits[c][subnet][n];
        _outstanding_classes[n][subnet][f->vc].push(c);
#endif
        
        dest_buf->SendingFlit(f);
        
        if(_pri_type == network_age_based) {
          f->pri = numeric_limits<int>::max() - _time;
          assert(f->pri >= 0);
        }
        
        if(f->watch) {
          *(icnt->gWatchOut) << icnt->get_cycle() << " | "
          << "node" << n << " | "
          << "Injecting flit " << f->id
          << " into subnet " << subnet
          << " at time " << _time
          << " with priority " << f->pri
          << "." << endl;
        }
        f->itime = _time;
        
        // Pass VC "back"
        if(!_input_queue[subnet][n][c].empty() && !f->tail) {
          Flit * const nf = _input_queue[subnet][n][c].front();
          nf->vc = f->vc;
        }
        
        if((_sim_state == warming_up) || (_sim_state == running)) {
          ++_sent_flits[c][n];
          if(f->head) {
            ++_sent_packets[c][n];
          }
        }
        
#ifdef TRACK_FLOWS
        ++_injected_flits[c][n];
#endif
        
        _net[subnet]->WriteFlit(f, n);
        
      }
    }
  }
  //Send the credit To the network
  for(unsigned subnet = 0; subnet < _subnets; ++subnet) {
    for(unsigned n = 0; n < _nodes; ++n) {
      map<int, Flit *>::const_iterator iter = flits[subnet].find(n);
      if(iter != flits[subnet].end()) {
        Flit * const f = iter->second;

        f->atime = _time;
        if(f->watch) {
          *(icnt->gWatchOut) << icnt->get_cycle() << " | "
          << "node" << n << " | "
          << "Injecting credit for VC " << f->vc
          << " into subnet " << subnet
          << "." << endl;
        }
        Credit * const c = Credit::New();
        c->vc.insert(f->vc);
        _net[subnet]->WriteCredit(c, n);
        
#ifdef TRACK_FLOWS
        ++_ejected_flits[f->cl][n];
#endif
        
        _RetireFlit(f, n);
      }
    }
    flits[subnet].clear();
    // _InteralStep here
    _net[subnet]->Evaluate( );
    _net[subnet]->WriteOutputs( );
  }
  
  ++_time;
  assert(_time);
  if(icnt->gTrace){
    cout<<"TIME "<<_time<<endl;
  }
  
}
