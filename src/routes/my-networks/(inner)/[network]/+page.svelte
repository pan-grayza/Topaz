<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { onMount } from 'svelte'
    import { listen } from '@tauri-apps/api/event'
    import { linked_paths, networks } from 'src/store'
    import { page } from '$app/state'
    import { goto } from '$app/navigation'
    import Button from 'src/components/Button.svelte'

    const currentPath = page.url.pathname
    let networkName = page.params.network
    let network = $networks.find((n: Network) => n.name === networkName)
    let serverGroups = $state([]) as ServerGroup[]

    async function getNetworks() {
        await invoke<Network[]>('read_private_networks')
            .then((updated_networks) => ($networks = updated_networks))
            .catch((e) => console.error(e))
    }
    async function getServers() {
        console.log(await invoke<ServerGroup[]>('get_servers', { networkName }))
        await invoke<ServerGroup[]>('get_servers', { networkName }).then(
            (newServerGroups) => (serverGroups = newServerGroups)
        )
    }
    onMount(() => {
        getNetworks()
        getServers()
    })

    async function startServer() {
        await invoke('start_file_server_command', {
            serverMode: 'LocalHost',
            network: network,
        }).then(async () => await getServers())
    }
    async function stopServer(id: number) {
        await invoke('stop_file_server_command', {
            id,
            networkName,
        })
        await getServers()
    }
</script>

<div>
    <h1>Current Network: {network?.name}</h1>
    <div class="flex flex-col gap-1">
        <Button onClick={startServer}>Start LocalHost server</Button>
        <Button onClick={startServer}>Start Internet server</Button>
    </div>
    <p>Servers:</p>
    <ul>
        {#each serverGroups as serverGroup}
            <li>
                <p>{serverGroup.id}</p>

                <ul>
                    {#each serverGroup.addresses as address}
                        <li>{address.ip}:{address.port}</li>
                    {/each}
                </ul>

                <Button onClick={() => stopServer(serverGroup.id)}>Stop</Button>
            </li>
        {/each}
    </ul>
    <ul>
        {#if network}
            {#each network.linked_paths as linked_path}
                <li>{linked_path.name}</li>
            {/each}
        {/if}
    </ul>
    <button onclick={() => goto(`${currentPath}/client`)}>I am client</button>
</div>
