#include <iostream>
#include <chrono>
#include <thread>

#include "cpu.h"
#include <SDL2/SDL.h>

static const int WINDOW_WIDTH  = 640;
static const int WINDOW_HEIGHT = 480;

static const void LOG(const int line) {
    std::cout << "log at " << line << '\n';
    return;
}

static const void LOG_ERR(const int line, const std::string&& msg=std::string{} ) {
    std::cout << "error " << msg << "at " << line << '\n';
    return;
}

int main() {
    std::cout << "testing cpu compilation and simple implementation\n";
    auto& t_cpu = Chip8_cpu::get_instance();
    std::cout << "instanciated chip8 cpu\n";
    std::cout << "lalaland\n";
    t_cpu.get()->exec_op(0x8014);
    
    /*
    if( SDL_Init(SDL_INIT_VIDEO) < 0) {
        LOG_ERR(__LINE__, std::string{"init failed "});
        return 1;
    }

    auto window = SDL_CreateWindow("chip8_em", 
        SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 
        WINDOW_WIDTH, WINDOW_HEIGHT, 0);

    bool keep_running{false};
    while(keep_running){
        break;
    }
    std::this_thread::sleep_for(std::chrono::seconds(3));
    SDL_DestroyWindow(window);
    SDL_Quit();
    */
    return 0;
}