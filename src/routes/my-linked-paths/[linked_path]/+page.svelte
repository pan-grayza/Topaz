<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { goto } from '$app/navigation'
    import { linked_paths } from 'src/store'
    import { page } from '$app/state'
    import Button from 'src/components/Button.svelte'

    async function remove_linked_path(linkedPathName: string) {
        await invoke('unlink_directory', { linkedPathName })
    }
    let linkedPathName = page.params.linked_path
    let linkedPath = $linked_paths.find(
        (linked_path: LinkedPath) => linked_path.name === linkedPathName
    )
</script>

<div>
    <h1>{linkedPathName}</h1>
    <Button
        onClick={() => {
            remove_linked_path(linkedPathName)
            goto('/')
        }}>Delete</Button
    >
</div>
