<script lang="ts">
    import { onMount } from "svelte";
    import * as Select from "$lib/components/ui/select";
    import { Input } from "$lib/components/ui/input";
    import { Button } from "$lib/components/ui/button";
    import { newlyCreatedCompanyId } from "$lib/stores/formState";

    type NamedEntity = { id: number; displayLabel: string };

    let {
        items,
        caller,
        createNew,
        value = $bindable<number | undefined>(undefined),
    }: {
        items: NamedEntity[];
        caller?: string;
        createNew?: string;
        value: number | undefined;
    } = $props();

    // -- State --
    let selectValue = $state<string>("");

    // -- Derived Logic --
    const noItemsAvailable = $derived(items.length === 0);

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
        if (value !== undefined) {
            selectValue = String(value);
        }
    });

    $effect(() => {
        if (selectValue) {
            value = Number(selectValue);
        }
    });

    // -- Listen for newly created company --
    onMount(() => {
        const unsubscribe = newlyCreatedCompanyId.subscribe((id) => {
            if (id !== undefined && items.length > 0) {
                const newCompany = items.find((c) => c.id === id);
                if (newCompany) {
                    value = id;
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
    </div>

    {#if !noItemsAvailable}
        <Button href={createUrl}>Create new</Button>
    {/if}
</div>
