# Not complex Music Player

## Feature

- play and pause
- shuffle playlist by default


## Developement plan

- [x] play song from command line argument
- [x] play/pause a song with program command
- [x] playlist
- [ ] queue
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
    -> add any song
    -> remove any song

IF play_queue not empty -> play until it empty
ELSE -> play from playlist

WHEN add song to queue -> remove it from playlist
WHEN played the song in queue -> remove from queue


PLAYER
x   -> play
x   -> pause
    -> seek
    -> next/prevoius song
    -> song info
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
    -> enqueue
    -> remove from queue
    -> player command
    -> show queue
x   -> show playlist
    -> show song list

----- output
player state => [song name / file name, playing time, song length, volume, play/pause]
queue
playlist
song_list
played song

```
</details>
