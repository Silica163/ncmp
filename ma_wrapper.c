#include "./miniaudio.h"
#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int playing;
    int ended;
    int pause;
} PlayerStatus;

typedef struct {
    PlayerStatus player;
    ma_context ctx;
    ma_context_config ctx_cfg;
    ma_device device;
    ma_device_config dev_cfg;
    ma_decoder decoder;
    ma_decoder_config decoder_cfg;
    ma_mutex mutex;
} Wrapper;

static Wrapper w;

#define SAMPLE_FORMAT   ma_format_f32
#define CHANNEL_COUNT   2
#define SAMPLE_RATE     48000

void data_callback(ma_device* pDevice, void* pOutput, const void* pInput, ma_uint32 frameCount){
    ma_mutex_lock(&w.mutex);
    float * f32out = (float*)pOutput;
    if(w.player.playing && !w.player.pause){
        ma_uint64 pFrameRead;
        ma_decoder_read_pcm_frames(&w.decoder, f32out, frameCount, &pFrameRead);
        if(pFrameRead < frameCount){
            w.player.ended = 1;
            w.player.playing = 0;
        }
    } else {
        for(int i = 0; i < frameCount; i ++){
            f32out[i] = 0;
        }
    }
    ma_mutex_unlock(&w.mutex);
}

int maw_init(){
    ma_result result;
    w.ctx_cfg = ma_context_config_init();
    w.decoder_cfg = ma_decoder_config_init(SAMPLE_FORMAT, CHANNEL_COUNT, SAMPLE_RATE);
    w.dev_cfg = ma_device_config_init(ma_device_type_playback);
    w.dev_cfg.playback.format   = SAMPLE_FORMAT;
    w.dev_cfg.playback.channels = CHANNEL_COUNT;
    w.dev_cfg.sampleRate        = SAMPLE_RATE;
    w.dev_cfg.dataCallback      = data_callback;
    w.dev_cfg.pUserData         = NULL;

    result = ma_context_init(NULL, 0, &w.ctx_cfg, &w.ctx);
    if (result != MA_SUCCESS) {
        printf("Could not init context\n");
        return result;
    }

    result = ma_device_init(&w.ctx, &w.dev_cfg, &w.device);
    if (result != MA_SUCCESS) {
        printf("Could not init device\n");
        return result;
    }
    
    result = ma_mutex_init(&w.mutex);
    if (result != MA_SUCCESS) {
        printf("Could not init mutex\n");
        return result;
    }

    result = ma_device_start(&w.device);
    if (result != MA_SUCCESS) {
        printf("Could not start device\n");
        return result;
    }

    return 0;
}

void maw_uninit(){
    ma_decoder_uninit(&w.decoder);
    ma_mutex_uninit(&w.mutex);
    ma_device_uninit(&w.device);
    ma_context_uninit(&w.ctx);
}

int maw_play(const char * file){
    ma_decoder_uninit(&w.decoder);
    ma_result result = ma_decoder_init_file(file, &w.decoder_cfg, &w.decoder);
    if (result != MA_SUCCESS) {
        printf("Could not decode from '%s'\n", file);
        return result;
    }
    w.player.playing = 1;
    w.player.ended = 0;
    w.player.pause = 0;
    return 0;
}

bool maw_is_ended(){
    return w.player.ended;
}

PlayerStatus * maw_get_player_status(){
    return &w.player;
}
