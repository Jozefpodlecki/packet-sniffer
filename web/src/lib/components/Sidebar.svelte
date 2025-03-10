<script lang="ts">
    import { IconGridDots, IconGauge, IconChartHistogram, IconAdjustments, IconBug, IconChevronLeft } from '@tabler/icons-svelte';
	import { onNavigate } from '$app/navigation';

    let isOpen = $state(true);
    let currentRoute = $state('');

    onNavigate(({ to }) => {
        currentRoute = to?.url.pathname || '/';
    });

    const menuItems = [
        { href: '/', icon: IconGauge, label: 'Dashboard' },
        { href: '/history', icon: IconChartHistogram, label: 'History' },
        { href: '/settings', icon: IconAdjustments, label: 'Settings' },
        { href: '/debug', icon: IconBug, label: 'Debug' }
    ];

    function onToggle() {
        isOpen = !isOpen;
    }

</script>

<div class="p-2 flex flex-col gap-4">
    <div class="cursor-pointer">
        {#if isOpen }
            <IconChevronLeft onclick={onToggle} size={40} stroke={1}/>
        {:else}
            <IconGridDots onclick={onToggle} size={40} stroke={1}/>
        {/if}
    </div>
    {#if isOpen }
        {#each menuItems as item}
            {@const isSelected = currentRoute === item.href}
            <a
                href={item.href} 
                aria-current={isSelected ? "page" : "false"}
                class="flex items-center transition-colors"
                style="color: {isSelected ? 'gray' : 'white'};"
            >
                <item.icon size={40} stroke={1}/>
            </a>
        {/each}
    {/if}
</div>