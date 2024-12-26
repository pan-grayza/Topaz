<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    // Components
    import TextInput from 'src/components/TextInput.svelte'
    import Button from 'src/components/Button.svelte'
    import HelpCircle from 'src/icons/help-circle.svelte'

    let newLinkedPathName = $state('')
    let newLinkedPathPath = $state('')
    let statusLinkedPath = $state('')
    let isHelpMenuOpen = false

    function isMobileDevice() {
        return /Mobi|Android|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
            navigator.userAgent
        )
    }

    async function select_directory() {
        try {
            // Invoke the Rust command and handle the response
            const result = await invoke<
                string | { Ok: string | null } | { Err: string }
            >('select_directory')

            if (typeof result === 'object' && result !== null) {
                if ('Ok' in result) {
                    if (result.Ok) {
                        newLinkedPathPath = result.Ok // Set the path if selection was successful
                        statusLinkedPath = 'Directory selected successfully!'
                    } else {
                        statusLinkedPath = 'No directory selected.'
                    }
                } else if ('Err' in result) {
                    statusLinkedPath = `Error: ${result.Err}` // Handle error case
                }
            } else if (typeof result === 'string') {
                // If a plain string (e.g., a path) is returned
                newLinkedPathPath = result
                statusLinkedPath = 'Directory selected successfully!'
            }
        } catch (error) {
            // Handle any potential invocation errors
            statusLinkedPath = `Error invoking select_directory: ${error}`
        }
    }

    async function link_directory() {
        if (typeof newLinkedPathName !== 'string') {
            statusLinkedPath = 'Invalid name'
            return
        }
        statusLinkedPath = await invoke('link_directory', {
            path: newLinkedPathPath,
            name: newLinkedPathName,
        })
    }
</script>

<div
    class="relative z-10 flex flex-col items-center justify-center w-full h-full p-2"
>
    <div class="relative flex flex-col gap-4 w-60 h-128">
        <p class="text-2xl">Link directory</p>

        <form
            class="relative flex flex-col gap-2 text-lg"
            onsubmit={(e) => {
                e.preventDefault()
                link_directory()
                newLinkedPathPath = ''
                newLinkedPathName = ''
            }}
        >
            <TextInput
                type="text"
                placeholder="Enter a name..."
                className="rounded-md"
                bind:value={newLinkedPathName}
            />

            <div class="relative flex flex-row w-full">
                {#if isMobileDevice()}
                    <TextInput
                        placeholder="Folder location"
                        bind:value={newLinkedPathPath}
                        className="w-full rounded-md"
                    />
                {:else}
                    <button
                        class="absolute z-10 w-full h-full"
                        onclick={(e) => {
                            e.preventDefault()
                            select_directory()
                        }}
                        aria-label="Select Directory"
                    ></button>
                    <TextInput
                        placeholder="Click to select Directory"
                        value={newLinkedPathPath}
                        disabled
                        className="w-full rounded-md"
                    />
                {/if}
            </div>

            <p>{statusLinkedPath}</p>
            <Button
                className="rounded-md "
                type="submit">Link directory</Button
            >
        </form>
    </div>
</div>
