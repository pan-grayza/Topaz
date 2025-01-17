<script lang="ts">
    interface Props {
        stroke_width?: number | string;
        size?: number | string;
        className?: any;
    }

    let { stroke_width = '2', size = '24', className = '' + 'feather feather-help-circle' }: Props = $props();
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    class={className}
    stroke="currentColor"
    stroke-width={stroke_width}
    stroke-linecap="round"
    stroke-linejoin="round"
    ><circle cx="12" cy="12" r="10" /><path
        d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"
    /><line x1="12" y1="17" x2="12.01" y2="17" /></svg
>
