<script lang="ts">
    import * as Select from "$lib/components/ui/select";
    import { Input } from "$lib/components/ui/input";
    import { Checkbox } from "$lib/components/ui/checkbox";
    import { Label } from "$lib/components/ui/label";

    type NamedEntity = { id: number; displayLabel: string };
    type ComponentState = [number | string | undefined, boolean];

    let {
        items,
        value = $bindable<ComponentState>([undefined, false]),
    }: {
        items: NamedEntity[];
        value: ComponentState;
    } = $props();

    // -- State --
    let selectValue = $state<string>("");
    let textValue = $state<string>("");

    // -- Derived Logic --
    const noItemsAvailable = $derived(items.length === 0);
    const isManualMode = $derived(value[1] || noItemsAvailable);

    const selectionOptions = $derived.by(() =>
        items.map((o) => ({
            label: o.displayLabel,
            value: String(o.id),
        })),
    );

    const triggerContent = $derived.by(() => {
        const option = selectionOptions.find((o) => o.value === selectValue);
        return option?.label ?? "Select option";
    });

    // -- Synchronization --
    // Sync external props to internal state
    $effect(() => {
        const [incomingVal, incomingIsManual] = value;

        if (incomingVal === undefined) return;

        if (incomingIsManual || noItemsAvailable) {
            if (typeof incomingVal === "string") {
                textValue = incomingVal;
            }
        } else if (typeof incomingVal === "number") {
            selectValue = String(incomingVal);
        }
    });

    // Sync internal select to external state
    $effect(() => {
        if (!isManualMode && selectValue) {
            value = [Number(selectValue), false];
        }
    });

    // Force manual mode when items become empty
    $effect(() => {
        if (noItemsAvailable && !value[1]) {
            value = [textValue, true];
        }
    });

    // -- Event Handlers --
    function handleInputChange(e: Event) {
        textValue = (e.target as HTMLInputElement).value;
        value = [textValue, true];
    }

    function handleCheckboxChange(checked: boolean) {
        if (checked) {
            value = [textValue, true];
        } else {
            value = [selectValue ? Number(selectValue) : undefined, false];
        }
    }
</script>

<div class="flex w-full items-center gap-2">
    <div class="flex-grow">
        {#if isManualMode}
            <Input
                type="text"
                placeholder={noItemsAvailable
                    ? "No items available.  Enter manually."
                    : "Enter custom value... "}
                value={textValue}
                oninput={handleInputChange}
            />
        {:else}
            <Select.Root type="single" bind:value={selectValue}>
                <Select.Trigger class="w-full">
                    {triggerContent}
                </Select.Trigger>
                <Select.Content class="w-full">
                    <Select.Group>
                        {#each selectionOptions as o (o.value)}
                            <Select.Item value={o.value}>{o.label}</Select.Item>
                        {/each}
                    </Select.Group>
                </Select.Content>
            </Select.Root>
        {/if}
    </div>

    {#if !noItemsAvailable}
        <div class="flex items-center space-x-2">
            <Checkbox
                id="manual-mode"
                checked={value[1]}
                onCheckedChange={handleCheckboxChange}
            />
            <Label for="manual-mode" class="text-sm font-medium cursor-pointer">
                Manual entry
            </Label>
        </div>
    {/if}
</div>
