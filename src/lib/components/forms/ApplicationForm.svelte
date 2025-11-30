<script lang="ts">
    import { Label } from "$lib/components/ui/label";
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import { Stage } from "$lib/types/enums";
    import { Button } from "$lib/components/ui/button";
    import { goto } from "$app/navigation";
    import {
        createApplication,
        updateApplication,
        applications,
    } from "$lib/stores/applications";
    import type { Application } from "$lib/types/application";
    import { jobListings } from "$lib/stores/jobListings";
    import CustomDatePicker from "./utils/CustomDatePicker.svelte";
    import { Textarea } from "../ui/textarea";
    import { newlyCreatedApplicationId } from "$lib/stores/formState";
    import CustomIDSelectCreate from "./utils/CustomIDSelectCreate.svelte";

    let {
        applicationID,
        callerChain = [],
    }: { applicationID?: number; callerChain?: string[] } = $props();

    let form = $state<Application>({
        jobListingId: undefined,
        stage: Stage.Applied,
        appliedDate: "",
        applicationNotes: "",
    });

    $effect(() => {
        if (!applicationID) return;
        const found = $applications.find((a) => a.id === applicationID);
        if (found) Object.assign(form, found);
    });

    const jobListingItems = $derived.by(() => {
        return $jobListings
            .filter((j) => j.id !== undefined)
            .map((j) => ({
                id: j.id!,
                displayLabel: j.title,
            }));
    });

    let jobListingId = $state<number | undefined>(undefined);
    const invalidSubmit = $derived(!form.appliedDate);
    $effect(() => {
        form.jobListingId = jobListingId;
    });
    async function submit() {
        if (invalidSubmit) return;

        let createdApplication: Application;

        if (applicationID) {
            await updateApplication(Number(applicationID), form);
            createdApplication = form;
        } else {
            createdApplication = await createApplication(form);
        }

        console.log($state.snapshot(form));
        if (callerChain.length > 0 && !applicationID) {
            newlyCreatedApplicationId.set(createdApplication.id);
        }

        // Go back: pop last item from chain
        if (callerChain.length > 1) {
            const backToCaller = callerChain[callerChain.length - 2];
            const newChain = callerChain.slice(0, -1);
            const params = new URLSearchParams();
            params.set("callerChain", newChain.join(","));
            goto(`/${backToCaller}/create?${params.toString()}`);
        } else if (callerChain.length === 1) {
            goto(`/${callerChain[0]}/create`);
        } else {
            goto("/applications");
        }
    }

    function handleCancel() {
        if (callerChain.length > 0) {
            goto(`/${callerChain[0]}/create`);
        } else {
            goto("/applications");
        }
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-4">
    <h1 class="text-xl font-extrabold">
        {applicationID ? "Edit Application" : "Create a new Application"}
    </h1>

    <div class="space-y-4">
        <div>
            <Label for="jobListing" class="py-2">Job Listing</Label>
            <CustomIDSelectCreate
                items={jobListingItems}
                bind:value={jobListingId}
                {callerChain}
                currentPage="applications"
                createNew="jobListings"
            />
        </div>

        <div>
            <Label for="stage" class="py-2">Stage</Label>
            <CustomEnumSelector
                enumObject={Stage}
                bind:selectedValue={form.stage as Stage | undefined}
            />
        </div>

        <div>
            <Label for="applicationDate" class="py-2 required"
                >Date Applied</Label
            >
            <CustomDatePicker bind:selectedDate={form.appliedDate} />
        </div>

        <div>
            <Label for="notes" class="py-2">Notes</Label>
            <Textarea
                id="notes"
                class="! min-h-30"
                bind:value={form.applicationNotes}
                placeholder="Add notes or context..."
            />
        </div>
    </div>

    <div class="flex justify-between mt-6">
        <Button variant="destructive" onclick={handleCancel}>Cancel</Button>
        <Button disabled={invalidSubmit} onclick={submit}>
            {applicationID ? "Update" : "Create"}
        </Button>
    </div>
</section>
