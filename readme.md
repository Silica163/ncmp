# Not complex Music Player

## Feature

- play, pause, seek
- queue
- shuffle playlist by default

## Usage

To use this program, download or clone this repo and build it with `make`.

```shell
./ncmp <audio files or directory>
```

Internal command
- play
- pause
- p (toggle play/pause)
- seek <sec> (seek to <sec>)
- enq, enqueue <song id> <queue index?> (add song to queue by id, position song at <queue index> if provided.)
- deq, dequeue <queue index?> (remove song from queue by \<index\>, remove first song in queue if index is not provided.)
- queue (show queue)
- info
- f, filelist (show filelist)
- r, remove (remove file form filelist)
- playlist (show playlist)
- q, exit, quit

## Developement plan

- [x] play song from command line argument
- [x] play/pause a song with program command
- [x] playlist
- [x] queue
- [ ] next/previous

<details>
<summary>More details...</summary>

```
song_list <- music_dir

song_list -> suffle -> playlist

song_list   -> add
            -> remove by dir

PLAYLIST

WHEN playlist is empty -> resuffle song_list and add it to playlist


QUEUE
x   -> add any song
x   -> remove any song

IF play_queue not empty -> play until it empty
ELSE -> play from playlist

WHEN add song to queue -> remove it from playlist
WHEN played the song in queue -> remove from queue


PLAYER
x   -> play
x   -> pause
x   -> seek
    -> next/prevoius song
x   -> song info
    -? volume control

[ ..., previous, current, next, ... ]
[played        ]          [ queue ][ playlist ]
played list
    -> add last
    -> remove last


----- data
Queue
Plyed song
Playlist
Song list
Player state

----- input
cmd
x   -> enqueue
x   -> remove from queue
    -> player command
x   -> show queue
x   -> show playlist
x   -> show song list

----- output
player state => [song name / file name, playing time, song length, volume, play/pause]
queue
playlist
song_list
played song

```
</details>
