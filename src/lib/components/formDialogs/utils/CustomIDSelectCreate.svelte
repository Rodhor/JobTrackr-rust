<script lang="ts">
    import { onMount } from "svelte";
    import * as Select from "$lib/components/ui/select";
    import { Input } from "$lib/components/ui/input";
    import { Button } from "$lib/components/ui/button";
    import { newlyCreatedCompanyId } from "$lib/stores/formState";

    type NamedEntity = { id: number; displayLabel: string };
    type ComponentState = [number | string | undefined, boolean];

    let {
        items,
        caller,
        createNew,
        value = $bindable<ComponentState>([undefined, false]),
    }: {
        items: NamedEntity[];
        caller?: string;
        createNew?: string;
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

    const createUrl = $derived.by(() => {
        const params = new URLSearchParams();
        if (caller) params.set("caller", caller);
        if (createNew) params.set("createNew", createNew);
        return `/${createNew}/create?${params.toString()}`;
    });

    // -- Synchronization --
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

    $effect(() => {
        if (!isManualMode && selectValue) {
            value = [Number(selectValue), false];
        }
    });

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

    // -- Listen for newly created company --
    onMount(() => {
        const unsubscribe = newlyCreatedCompanyId.subscribe((id) => {
            if (id !== undefined && items.length > 0) {
                const newCompany = items.find((c) => c.id === id);
                if (newCompany) {
                    value = [id, false];
                    selectValue = String(id);
                    newlyCreatedCompanyId.set(undefined); // Reset
                }
            }
        });

        return unsubscribe;
    });
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
        <Button href={createUrl}>Create new</Button>
    {/if}
</div>
