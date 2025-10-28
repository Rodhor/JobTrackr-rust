<script lang="ts">
    import * as Select from "$lib/components/ui/select";

    type EnumLike = Record<string, string | number>;

    let {
        enumObject,
        selectedValue = $bindable(""),
        label = "Select option",
    }: {
        enumObject: EnumLike;
        selectedValue: string;
        label?: string;
    } = $props();

    // Convert enum to label/value pairs
    const selectionOptions = $derived(() =>
        Object.entries(enumObject)
            .filter(([key, value]) => isNaN(Number(key))) // filter numeric reverse mappings
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
    <Select.Trigger class="w-[180px]">{triggerContent()}</Select.Trigger>
    <Select.Content>
        <Select.Group>
            <Select.Label>{label}</Select.Label>
            {#each selectionOptions() as o (o.value)}
                <Select.Item value={o.value}>{o.label}</Select.Item>
            {/each}
        </Select.Group>
    </Select.Content>
</Select.Root>
