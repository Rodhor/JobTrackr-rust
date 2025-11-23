<script lang="ts">
    import * as Select from "$lib/components/ui/select";

    type EnumLike = Record<string, string | number>;

    let {
        enumObject,
        selectedValue = $bindable<string | undefined>(undefined),
        label = "Select option",
    }: {
        enumObject: EnumLike;
        selectedValue: string | undefined;
        label?: string;
    } = $props();

    const selectionOptions = $derived(() =>
        Object.entries(enumObject)
            .filter(([key]) => isNaN(Number(key)))
            .map(([key, value]) => ({
                label: key.replace(/_/g, " "),
                value: String(value),
            })),
    );

    const triggerContent = $derived(() => {
        const match = selectionOptions().find((o) => o.value === selectedValue);
        return match ? match.label : label;
    });
</script>

<Select.Root type="single" bind:value={selectedValue}>
    <Select.Trigger class="w-full">{triggerContent()}</Select.Trigger>
    <Select.Content class="w-full">
        <Select.Group>
            {#each selectionOptions() as o (o.value)}
                <Select.Item value={o.value}>{o.label}</Select.Item>
            {/each}
        </Select.Group>
    </Select.Content>
</Select.Root>
