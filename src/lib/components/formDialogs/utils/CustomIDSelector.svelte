<script lang="ts">
    import * as Select from "$lib/components/ui/select";

    type NamedEntity = { id: number; displayLabel: string };

    let {
        items,
        selectedId = $bindable<number | undefined>(undefined),
    }: {
        items: NamedEntity[];
        selectedId: number | undefined;
    } = $props();

    let internalValue = $state(
        selectedId !== undefined ? String(selectedId) : "",
    );

    $effect(() => {
        selectedId = internalValue !== "" ? Number(internalValue) : undefined;
    });

    const selectionOptions = $derived(() =>
        items.map((o) => ({
            label: o.displayLabel,
            value: String(o.id),
        })),
    );

    const triggerContent = $derived(() => {
        const match = selectionOptions().find(
            (o) => Number(o.value) === selectedId,
        );
        return match ? match.label : "Select option";
    });
</script>

{#if items.length === 0}
    <div class="text-sm text-muted-foreground">No items available</div>
{:else}
    <Select.Root type="single" bind:value={internalValue}>
        <Select.Trigger class="w-full">{triggerContent()}</Select.Trigger>
        <Select.Content class="w-full">
            <Select.Group>
                {#each selectionOptions() as o (o.value)}
                    <Select.Item value={o.value}>{o.label}</Select.Item>
                {/each}
            </Select.Group>
        </Select.Content>
    </Select.Root>
{/if}
