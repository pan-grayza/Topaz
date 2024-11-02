<script lang="ts">
    interface Props {
        value?: string;
        className?: string;
        placeholder?: string;
        disabled?: boolean;
        [key: string]: any
    }

    let {
        value = $bindable(''),
        className = '',
        placeholder = '',
        disabled = false,
        ...rest
    }: Props = $props();
</script>

{#if !disabled}
    <input
        type="text"
        {placeholder}
        bind:value
        class={'px-2 py-1 relative bg-DarkModeTextInputBG placeholder:text-neutral-600' +
            ' ' +
            className}
        {...rest}
    />
{:else}
    <input
        type="text"
        {placeholder}
        {value}
        class={'px-2 py-1 relative cursor-not-allowed bg-neutral-700 placeholder:text-neutral-500' +
            ' ' +
            className}
        disabled
        {...rest}
    />
{/if}
