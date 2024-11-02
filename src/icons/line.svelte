<script lang="ts">
    interface Props {
        stroke_width?: number | string;
        size?: number | string;
        className?: any;
    }

    let { stroke_width = '1', size = '24', className = '' + 'feather feather-line' }: Props = $props();
</script>

<svg
    width={size}
    height={size}
    stroke="currentColor"
    stroke-width={stroke_width}
    class={className}
    stroke-linecap="round"
    stroke-linejoin="round"
    xmlns="http://www.w3.org/2000/svg"
>
    <line x1="30%" y1="50%" x2="70%" y2="50%" />
</svg>
