<script lang="ts">
    import { onMount } from "svelte";
    import * as Select from "$lib/components/ui/select";
    import { Button } from "$lib/components/ui/button";
    import {
        newlyCreatedCompanyId,
        newlyCreatedPersonId,
        newlyCreatedJobListingId,
        newlyCreatedApplicationId,
        newlyCreatedInteractionId,
    } from "$lib/stores/formState";

    type NamedEntity = { id: number; displayLabel: string };

    let {
        items,
        callerChain = [],
        createNew,
        currentPage,
        disabled = false,
        value = $bindable<number | undefined>(undefined),
    }: {
        items: NamedEntity[];
        callerChain?: string[];
        createNew?: string;
        currentPage?: string;
        disabled?: boolean;
        value: number | undefined;
    } = $props();

    // -- State --
    let selectValue = $state<string>("");

    // -- Derived Logic --
    const noItemsAvailable = $derived(items.length === 0);
    const isDisabled = $derived(noItemsAvailable || disabled);

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
        // Add current page to chain
        const newChain = currentPage
            ? [...callerChain, currentPage]
            : callerChain;
        console.log("===== CustomIDSelectCreate =====");
        console.log("currentPage:", currentPage);
        console.log("callerChain received:", callerChain);
        console.log("newChain built:", newChain);
        params.set("callerChain", newChain.join(","));
        const finalUrl = `/${createNew}/create?${params.toString()}`;
        console.log("Final URL:", finalUrl);
        console.log("===== End CustomIDSelectCreate =====");
        return finalUrl;
    });

    // Map createNew type to corresponding store
    const storeMap = {
        companies: newlyCreatedCompanyId,
        people: newlyCreatedPersonId,
        jobListings: newlyCreatedJobListingId,
        applications: newlyCreatedApplicationId,
        interactions: newlyCreatedInteractionId,
    };

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

    // -- Listen for newly created items --
    onMount(() => {
        const store = storeMap[createNew as keyof typeof storeMap];
        if (!store) return;

        const unsubscribe = store.subscribe((id) => {
            if (id !== undefined && items.length > 0) {
                const newItem = items.find((item) => item.id === id);
                if (newItem) {
                    value = id;
                    selectValue = String(id);
                    store.set(undefined); // Reset
                }
            }
        });

        return unsubscribe;
    });
</script>

<div class="flex w-full items-center gap-2">
    <div class="flex-grow">
        <Select.Root
            type="single"
            bind:value={selectValue}
            disabled={isDisabled}
        >
            <Select.Trigger class="w-full">
                {isDisabled
                    ? noItemsAvailable
                        ? "No options available"
                        : "Select option"
                    : triggerContent}
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

    <Button href={createUrl}>Create new</Button>
</div>
