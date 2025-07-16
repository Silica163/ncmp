# Not complex Music Player

## Feature

- play, pause, seek
- skip to next song, play previous song
- queue
- played history
- shuffle playlist by default

## Usage

To use this program, download or clone this repo and build it with `make`.

```shell
./ncmp <audio files or directory>
```

## Internal Command

### Player

- play
- pause
- p (toggle play/pause)
- seek <sec> (seek to <sec>)
- playlist (show playlist)
- hist, history
- next, n
- previous, prev
- q, exit, quit
- info

### Queue

- queue (show queue)
- enq, enqueue <song indices>
- deq, dequeue <queue index?> (remove song from queue by \<index\>, remove first song in queue if index is not provided.)
- mvq, movequeue \<from\> \<to\>

### Files

- f, files (show filelist)
- r, remove (remove file form filelist)
- rp, remove\_pattern <case sensitive pattern>

## Developement plan

- [x] play song from command line argument
- [x] play/pause a song with program command
- [x] playlist
- [x] queue
- [x] next/previous

<details>
<summary>More details...</summary>

```
song_list <- music_dir

song_list -> suffle -> playlist

song_list   -> add
x           -> remove by dir (pattern)

PLAYLIST

WHEN playlist is empty -> resuffle song_list and add it to playlist


QUEUE
x   -> add any song
x   -> remove any song

IF play_queue not empty -> play until it empty
ELSE -> play from playlist

WHEN played the song in queue -> remove from queue


PLAYER
x   -> play
x   -> pause
x   -> seek
x   -> next/prevoius song
x   -> song info

[ ..., previous, current, next, ... ]
[played        ]          [ queue ][ playlist ]
played list
x   -> add last
x   -> remove last


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
x   -> player command
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
