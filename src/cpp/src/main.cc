#include "main_lib.h"
#include "spdlog/spdlog.h"
int main(int argc, char **argv)
{
    spdlog::info("argc: {}", argc);
    for (int i = 0; i < argc; i++)
    {
        spdlog::info("argv[{}]: {}", i, argv[i]);
    }
    return _main(argc, argv);
}