<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';

  let url = '';
  let format = 'mp4';
  let start = '';
  let end = '';
  let status = '';
  let progress = 0;
  let unlisten;

  async function download() {
    status = 'Downloading...';
    progress = 0;
    try {
      await invoke('download_video', { url, format, trim_start: start || null, trim_end: end || null });
      status = 'Download complete';
      progress = 1;
    } catch (e) {
      status = 'Error: ' + e;
    }
  }

  onMount(async () => {
    unlisten = await listen('progress', (e) => {
      progress = e.payload / 100;
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

<main>
  <h1>Downflix</h1>
  <input placeholder="YouTube URL" bind:value={url} />
  <select bind:value={format}>
    <option value="mp4">MP4</option>
    <option value="mp3">MP3</option>
    <option value="bestaudio">Best Audio</option>
  </select>
  <div>
    <input placeholder="Start time (optional)" bind:value={start} />
    <input placeholder="End time (optional)" bind:value={end} />
  </div>
  <button on:click={download}>Download</button>
  {#if status}
    <p>{status}</p>
  {/if}
  {#if progress}
    <progress max="1" value={progress}></progress>
  {/if}
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: 500px;
    margin: 2rem auto;
  }
</style>
