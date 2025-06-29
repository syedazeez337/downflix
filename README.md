# Downflix

Downflix is a cross-platform desktop application built with [Svelte](https://svelte.dev/) and [Tauri](https://tauri.app/) with backend logic in Rust. It provides a simple interface for downloading YouTube videos using `yt-dlp`.

## Features

- Paste a YouTube URL and choose the desired output format (MP4, MP3, best audio).
- Optional start and end times for trimming.
- Invokes `yt-dlp` through Tauri's Rust backend.

## Development

```
npm install
npm run dev
```

In another terminal:

```
cargo tauri dev
```

Ensure `yt-dlp` is installed and available in your `PATH`.
