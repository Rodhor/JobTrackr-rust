<script lang="ts">
    import * as Select from "$lib/components/ui/select";
    import type { Readable } from "svelte/store";

    type NamedEntity = { id: number; name: string };
    type Option = { value: number; label: string };

    let {
        sourceStore,
        selectedId = $bindable<number | null>(null),
    }: {
        sourceStore: Readable<NamedEntity[]>;
        selectedId: number | null;
    } = $props();

    let internalValue = $state(selectedId ? String(selectedId) : "");

    // new syntax: $derived returns a function
    const derivedId = $derived(() =>
        internalValue ? Number(internalValue) : null,
    );

    // sync to parent (call derivedId())
    $effect(() => {
        selectedId = derivedId();
    });

    const selectionOptions = $derived(() =>
        $sourceStore.map(
            (o: NamedEntity): Option => ({
                value: o.id,
                label: o.name,
            }),
        ),
    );

    const triggerContent = $derived(() => {
        const match = selectionOptions().find((o) => o.value === derivedId());
        return match ? match.label : "Select option";
    });
</script>

<Select.Root type="single" bind:value={internalValue}>
    <Select.Trigger class="w-[180px]">{triggerContent()}</Select.Trigger>
    <Select.Content>
        <Select.Group>
            <Select.Label>Selection</Select.Label>
            {#each selectionOptions() as o (o.value)}
                <Select.Item value={String(o.value)}>{o.label}</Select.Item>
            {/each}
        </Select.Group>
    </Select.Content>
</Select.Root>
