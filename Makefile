RUST_SDL_PATH=.
SDL_LIB_PATH=/usr/local/Cellar/sdl/1.2.15/lib/
SDL_INCLUDE_PATH=/usr/local/Cellar/sdl/1.2.15/include/SDL
SDL_MAIN_PATH=/usr/local/Cellar/sdl/1.2.15/libexec/SDLMain.m

all:
	rustc -L$(RUST_SDL_PATH) -L$(SDL_LIB_PATH) -C link-args=" -I$(SDL_INCLUDE_PATH) -framework CoreFoundation -framework CoreGraphics -framework AppKit $(SDL_MAIN_PATH) " worm.rs
