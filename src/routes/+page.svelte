<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Keycode from '../lib/Models/Keycode';
	let currentKeybind = Keycode.B;

	async function updateKeybind() {
		console.log('sending');
		await invoke('map_key', { input: currentKeybind });
		console.log('sent');
	}
</script>

<select bind:value={currentKeybind} on:change={updateKeybind}>
	{#each Object.entries(Keycode) as [key, value]}
		<option {value}>{key}</option>
	{/each}
</select>
