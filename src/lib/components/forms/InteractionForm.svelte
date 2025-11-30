<script lang="ts">
    import { goto } from "$app/navigation";
    import { Button } from "$lib/components/ui/button";
    import { applications } from "$lib/stores/applications";
    import { people } from "$lib/stores/people";
    import { companies } from "$lib/stores/companies";
    import {
        createInteraction,
        interactions,
        updateInteraction,
    } from "$lib/stores/interactions";
    import { InteractionType } from "$lib/types/enums";
    import type { Interaction } from "$lib/types/interaction";
    import CustomEnumSelector from "../formDialogs/utils/CustomEnumSelector.svelte";
    import CustomIDSelectCreate from "../formDialogs/utils/CustomIDSelectCreate.svelte";
    import CustomDatePicker from "../formDialogs/utils/CustomDatePicker.svelte";
    import { Input } from "../ui/input";
    import { Label } from "../ui/label";
    import { Textarea } from "../ui/textarea";
    import {
        newlyCreatedInteractionId,
        newlyCreatedApplicationId,
        newlyCreatedPersonId,
        newlyCreatedCompanyId,
    } from "$lib/stores/formState";
    import { onMount } from "svelte";

    let {
        interactionID,
        callerChain = [],
    }: { interactionID?: number; callerChain?: string[] } = $props();

    const applicationItems = $derived(
        $applications
            .filter((a) => a.id !== undefined)
            .map((a) => ({
                id: a.id!,
                displayLabel: `Application #${a.id}`,
            })),
    );

    const personItems = $derived(
        $people
            .filter((p) => p.id !== undefined)
            .map((p) => ({
                id: p.id!,
                displayLabel: `${p.firstName} ${p.lastName}`,
            })),
    );

    const companyItems = $derived(
        $companies
            .filter((c) => c.id !== undefined)
            .map((c) => ({
                id: c.id!,
                displayLabel: c.name || `Company #${c.id}`,
            })),
    );

    let form = $state<Interaction>({
        interactionType: InteractionType.Email,
        interactionDate: new Date().toISOString().split("T")[0],
        subject: "",
        summary: "",
        medium: "",
        applicationId: undefined,
        personId: undefined,
        companyId: undefined,
    });

    // Separate state variables for binding
    let applicationId = $state<number | undefined>(undefined);
    let personId = $state<number | undefined>(undefined);
    let companyId = $state<number | undefined>(undefined);
    let interactionDate = $state<string>(
        new Date().toISOString().split("T")[0],
    );

    // Sync state variables to form
    $effect(() => {
        form.applicationId = applicationId;
    });

    $effect(() => {
        form.personId = personId;
    });

    $effect(() => {
        form.companyId = companyId;
    });

    $effect(() => {
        form.interactionDate = interactionDate;
    });

    $effect(() => {
        if (!interactionID) return;
        const found = $interactions.find((i) => i.id === interactionID);
        if (found) {
            Object.assign(form, found);
            // Sync to state variables too
            if (found.applicationId) applicationId = found.applicationId;
            if (found.personId) personId = found.personId;
            if (found.companyId) companyId = found.companyId;
            interactionDate = found.interactionDate;
        }
    });

    const invalidSubmit = $derived(!form.subject || !form.summary);

    // Listen for newly created items when returning from nested forms
    onMount(() => {
        const unsubscribers: (() => void)[] = [];

        // Subscribe to application store
        unsubscribers.push(
            newlyCreatedApplicationId.subscribe((id) => {
                if (id !== undefined) {
                    applicationId = id;
                    setTimeout(
                        () => newlyCreatedApplicationId.set(undefined),
                        0,
                    );
                }
            }),
        );

        // Subscribe to person store
        unsubscribers.push(
            newlyCreatedPersonId.subscribe((id) => {
                if (id !== undefined) {
                    personId = id;
                    setTimeout(() => newlyCreatedPersonId.set(undefined), 0);
                }
            }),
        );

        // Subscribe to company store
        unsubscribers.push(
            newlyCreatedCompanyId.subscribe((id) => {
                if (id !== undefined) {
                    companyId = id;
                    setTimeout(() => newlyCreatedCompanyId.set(undefined), 0);
                }
            }),
        );

        return () => {
            unsubscribers.forEach((unsub) => unsub());
        };
    });

    async function submit() {
        if (invalidSubmit) return;

        let createdInteraction: Interaction;

        if (interactionID) {
            await updateInteraction(Number(interactionID), form);
            createdInteraction = form;
        } else {
            createdInteraction = await createInteraction(form);
        }

        if (callerChain.length > 0 && !interactionID) {
            newlyCreatedInteractionId.set(createdInteraction.id);
        }

        // Go back to where we came from
        if (callerChain.length > 0) {
            // Get the page we came from (last item in chain)
            const backToCaller = callerChain[callerChain.length - 1];
            // Build new chain WITHOUT the current page (remove last item)
            const newChain = callerChain.slice(0, -1);

            if (newChain.length > 0) {
                // Pass remaining chain to previous page
                const params = new URLSearchParams();
                params.set("callerChain", newChain.join(","));
                const url = `/${backToCaller}/create?${params.toString()}`;
                await goto(url);
            } else {
                // No more chain, just go back to caller
                const url = `/${backToCaller}/create`;
                await goto(url);
            }
        } else {
            await goto("/interactions");
        }
    }

    function handleCancel() {
        if (callerChain.length > 0) {
            const url = `/${callerChain[0]}/create`;
            goto(url);
        } else {
            goto("/interactions");
        }
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-4">
    <h1 class="text-xl font-extrabold">
        {interactionID ? "Edit Interaction" : "Create a new Interaction"}
    </h1>

    <div class="space-y-4">
        <!-- Subject -->
        <div>
            <Label for="subject" class="py-2 required">Subject</Label>
            <Input
                id="subject"
                bind:value={form.subject}
                placeholder="Call with HR"
            />
        </div>
        <!-- Date & Type Row -->
        <div class="grid grid-cols-2 gap-4">
            <div>
                <Label for="interactionDate" class="py-2 required">Date</Label>
                <CustomDatePicker bind:selectedDate={interactionDate} />
            </div>
            <div>
                <Label for="interactionType" class="py-2 required">Type</Label>
                <CustomEnumSelector
                    enumObject={InteractionType}
                    bind:selectedValue={form.interactionType}
                />
            </div>
        </div>

        <!-- Medium -->
        <div>
            <Label for="medium" class="py-2">Medium</Label>
            <Input
                id="medium"
                bind:value={form.medium}
                placeholder="Zoom / Email / Office"
            />
        </div>

        <!-- Company -->
        <div>
            <Label for="company" class="py-2">Company</Label>
            <CustomIDSelectCreate
                items={companyItems}
                bind:value={companyId}
                {callerChain}
                currentPage="interactions"
                createNew="companies"
            />
        </div>

        <!-- Application -->
        <div>
            <Label for="application" class="py-2">Application</Label>
            <CustomIDSelectCreate
                items={applicationItems}
                bind:value={applicationId}
                {callerChain}
                currentPage="interactions"
                createNew="applications"
            />
        </div>

        <!-- Person -->
        <div>
            <Label for="person" class="py-2">Person</Label>
            <CustomIDSelectCreate
                items={personItems}
                bind:value={personId}
                {callerChain}
                currentPage="interactions"
                createNew="people"
            />
        </div>

        <!-- Summary -->
        <div>
            <Label for="summary" class="py-2 required">Summary</Label>
            <Textarea
                id="summary"
                bind:value={form.summary}
                placeholder="Details about the interaction..."
                rows={6}
            />
        </div>
    </div>

    <div class="flex justify-between mt-6">
        <Button variant="destructive" onclick={handleCancel}>Cancel</Button>
        <Button disabled={invalidSubmit} onclick={submit}>
            {interactionID ? "Update" : "Create"}
        </Button>
    </div>
</section>
