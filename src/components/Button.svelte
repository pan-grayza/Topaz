<script lang="ts">
    interface Props {
        className?: string;
        onClick?: any;
        styleType?: 'primary' | 'secondary' | 'tertiary' | 'danger';
        children?: import('svelte').Snippet;
        [key: string]: any
    }

    let {
        className = '',
        onClick = () => {},
        styleType = 'primary',
        children,
        ...rest
    }: Props = $props();
    let buttonClass =
        $derived(styleType === 'primary'
            ? 'px-2 py-1 bg-DarkModePrimBtn hover:bg-DarkModePrimBtnHover text-DarkModePrimBtnText'
            : styleType === 'secondary'
              ? 'px-2 py-1 bg-DarkModeSecBtn hover:bg-DarkModeSecBtnHover text-DarkModeSecBtnText'
              : styleType === 'tertiary'
                ? 'px-2 py-1 bg-DarkModeTriBtn hover:bg-DarkModeTriBtnHover text-DarkModeTriBtnText'
                : 'px-2 py-1 bg-DarkModeDangerBtn text-white hover:bg-DarkModeDangerBtnHover text-DarkModeDangerBtnText')
</script>

<button
    class={buttonClass + ' ' + className}
    onmouseup={onClick}
    {...rest}>{@render children?.()}</button
>
