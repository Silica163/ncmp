#include "./miniaudio.h"
#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int playing;
    int ended;
    int pause;
} PlayerStatus;

typedef struct {
    PlayerStatus * player;
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
    if(w.player->playing && !w.player->pause){
        ma_uint64 pFrameRead;
        ma_decoder_read_pcm_frames(&w.decoder, f32out, frameCount, &pFrameRead);
        if(pFrameRead < frameCount){
            w.player->ended = 1;
            w.player->playing = 0;
        }
    } else {
        for(int i = 0; i < frameCount; i ++){
            f32out[i] = 0;
        }
    }
    ma_mutex_unlock(&w.mutex);
}

int maw_init(PlayerStatus * player){
    if(player == NULL){
        printf("pointer to PlayerStatus is NULL, please give me something else.");
        exit(1);
    }
    w.player = player;

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
        printf("Could not init context: %d\n", result);
        return result;
    }

    result = ma_device_init(&w.ctx, &w.dev_cfg, &w.device);
    if (result != MA_SUCCESS) {
        printf("Could not init device: %d\n", result);
        return result;
    }

    result = ma_mutex_init(&w.mutex);
    if (result != MA_SUCCESS) {
        printf("Could not init mutex: %d\n", result);
        return result;
    }

    result = ma_device_start(&w.device);
    if (result != MA_SUCCESS) {
        printf("Could not start device: %d\n", result);
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
    w.player->playing = 1;
    w.player->ended = 0;
    w.player->pause = 0;
    return 0;
}

bool maw_is_ended(){
    return w.player->ended;
}

PlayerStatus * maw_get_player_status(){
    return w.player;
}

int maw_get_length_in_secs(){
    ma_uint64 frames = 0;
    ma_result r = ma_data_source_get_length_in_pcm_frames(&w.decoder, &frames);
    if(r != MA_SUCCESS){
        printf("Could not get length: %d\n", r);
        return -1;
    }
    int length = frames / SAMPLE_RATE;
    return length;
}

int maw_get_cursor_in_secs(){
    ma_uint64 frames = 0;
    ma_result r = ma_data_source_get_cursor_in_pcm_frames(&w.decoder, &frames);
    if(r != MA_SUCCESS){
        printf("Could not get cursor: %d\n", r);
        return -1;
    }
    int cursor = frames / SAMPLE_RATE;
    return cursor;
}

int maw_seek_to_sec(int target_sec){
    ma_uint64 avaliable_frames = 0;
    ma_result r = ma_data_source_get_length_in_pcm_frames(&w.decoder, &avaliable_frames);
    if(r != MA_SUCCESS){
        printf("Could not get length: %d\n", r);
        return -1;
    }

    ma_uint64 target_frame = target_sec * SAMPLE_RATE;
    if(target_frame >= avaliable_frames){
        printf("Could not seek beyond end of data source: %u > %u\n", target_frame, avaliable_frames);
        return -2;
    }
    r = ma_data_source_seek_to_pcm_frame(&w.decoder, target_frame);
    if(r != MA_SUCCESS){
        printf("Could not seek to %d: %d\n", target_sec, r);
        return -3;
    }
    return 0;
}
