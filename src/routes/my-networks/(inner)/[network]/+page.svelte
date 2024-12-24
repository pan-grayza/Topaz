<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { onMount } from 'svelte'
    import { listen } from '@tauri-apps/api/event'
    import { linked_paths, networks } from 'src/store'
    import { page } from '$app/stores'

    let serverStatus = $state('Not running')
    let serverAddresses = $state([]) as string[]

    async function get_networks() {
        await invoke<Network[]>('read_private_networks')
            .then((updated_networks) => ($networks = updated_networks))
            .catch((e) => console.error(e))
    }
    async function remove_network(network: Network) {
        await invoke('remove_network', { network })
    }
    onMount(() => {
        get_networks()
    })
    let networkName = $page.params.network
    let network = $networks.find((n: Network) => n.name === networkName)
    async function handleServer() {
        console.log(serverStatus)
        if (serverStatus === 'Server started!') {
            serverStatus = await invoke('stop_file_server_command')
        } else {
            let { status, addresses } = await invoke<LocalServerResponse>(
                'start_file_server_command',
                {
                    serverMode: 'LocalHost',
                    linkedPaths: network?.linked_paths,
                }
            )
            serverStatus = status
            serverAddresses = addresses
        }
    }
</script>

<div>
    <h1>Current Network: {network?.name}</h1>
    <button onclick={handleServer}
        >{#if serverStatus === 'Server started!'}
            Stop server
        {:else}
            Start server
        {/if}
    </button>
    <p>Status:{serverStatus}</p>
    <p>Addresses{serverAddresses}</p>
    <ul>
        {#if network}
            {#each network.linked_paths as linked_path}
                <li>{linked_path.name}</li>
            {/each}
        {/if}
    </ul>
</div>
