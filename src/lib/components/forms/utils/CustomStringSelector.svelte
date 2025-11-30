<script lang="ts">
    import * as Select from "$lib/components/ui/select";
    import type { Readable } from "svelte/store";

    type NamedEntity = { id: number; displayLabel: string };
    type Option = { value: number; label: string };

    let {
        sourceStore,
        selectedValue = $bindable(""),
    }: {
        sourceStore: Readable<NamedEntity[]>;
        selectedValue: string;
    } = $props();

    const selectionOptions = $derived(
        $sourceStore.map(
            (o: NamedEntity): Option => ({
                value: o.id,
                label: o.displayLabel,
            }),
        ),
    );

    const selectedId = $derived(selectedValue ? Number(selectedValue) : null);

    const triggerContent = $derived(
        selectionOptions.find((o: Option) => o.value === selectedId)?.label ??
            "Select option",
    );
</script>

<Select.Root type="single" bind:value={selectedValue}>
    <Select.Trigger class="w-full">
        {triggerContent}
    </Select.Trigger>
    <Select.Content class="w-full">
        <Select.Group>
            {#each selectionOptions as o (o.value)}
                <Select.Item value={String(o.value)}>{o.label}</Select.Item>
            {/each}
        </Select.Group>
    </Select.Content>
</Select.Root>
