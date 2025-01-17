<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { onMount } from 'svelte'
    import { networks } from 'src/store'
    import { page } from '$app/state'
    import TextInput from 'src/components/TextInput.svelte'
    import Button from 'src/components/Button.svelte'

    let response = $state()
    let address = $state('')
    let networkName = page.params.network
    let network = $networks.find((n: Network) => n.name === networkName)

    async function getHostLinkedPaths() {
        response = await invoke('get_host_linked_paths', { address })
    }
</script>

<div>
    <h1>Current Network: {network?.name}</h1>
    <TextInput
        placeholder="Address"
        bind:value={address}
        className="w-full rounded-md"
    />
    <Button onClick={() => getHostLinkedPaths()}>Connect</Button>
    {response}
</div>
