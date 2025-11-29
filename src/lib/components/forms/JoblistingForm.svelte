<script lang="ts">
    import { goto } from "$app/navigation";
    import CustomIDSelectCreate from "$lib/components/formDialogs/utils/CustomIDSelectCreate.svelte";
    import { Button } from "$lib/components/ui/button";
    import { companies, createCompany } from "$lib/stores/companies";
    import { newlyCreatedApplicationId } from "$lib/stores/formState";
    import {
        createJobListing,
        jobListings,
        updateJobListing,
    } from "$lib/stores/jobListings";
    import { Currency, SeniorityLevel, WorkType } from "$lib/types/enums";
    import type { JobListing } from "$lib/types/jobListing";
    import CustomEnumSelector from "../formDialogs/utils/CustomEnumSelector.svelte";
    import { Input } from "../ui/input";
    import { Label } from "../ui/label";
    import { Textarea } from "../ui/textarea";

    let {
        joblistingID,
        caller,
    }: { joblistingID?: number; caller?: string | null } = $props();

    // Prepare items for the component
    const items = $derived(
        $companies
            .filter((c) => c.id !== undefined)
            .map((c) => ({
                id: c.id!,
                displayLabel: c.name,
            })),
    );

    let form = $state<JobListing>({
        companyId: undefined,
        title: "",
        workType: WorkType.Other,
        category: "",
        seniorityLevel: SeniorityLevel.Mid,
        salaryMin: undefined,
        salaryMax: undefined,
        currency: Currency.EUR,
        description: "",
        url: "",
    });

    let companyId = $state<number | undefined>(undefined);

    $effect(() => {
        if (!joblistingID) return;
        const found = $jobListings.find((j) => j.id === joblistingID);
        if (found) {
            Object.assign(form, found);
            // Also sync the company selector
            if (found.companyId) {
                companyId = found.companyId;
            }
        }
    });

    const invalidSubmit = $derived(companyId === undefined);

    async function submit() {
        if (invalidSubmit) return;

        let createdListing: JobListing;

        // Create or update job listing
        if (joblistingID) {
            await updateJobListing(Number(joblistingID), form);
            createdListing = form;
        } else {
            createdListing = await createJobListing(form);
        }

        console.log($state.snapshot(form));
        if (caller && !joblistingID) {
            newlyCreatedApplicationId.set(createdListing.id);
        }

        if (caller) {
            goto(`/${caller}/create`);
        } else {
            goto("/jobListings");
        }
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-3">
    <h1 class="text-xl font-extrabold py-4">Create a new Job Listing</h1>

    <div>
        <Label class="py-2 required">Company:</Label>
        <CustomIDSelectCreate
            {items}
            bind:value={companyId}
            caller="jobListings"
            createNew="companies"
        />
    </div>

    <div>
        <Label for="title" class="py-2 required">Title</Label>
        <Input
            id="title"
            bind:value={form.title}
            placeholder="Position title"
        />
    </div>

    <div>
        <Label for="workType" class="py-2">Work Type</Label>
        <CustomEnumSelector
            enumObject={WorkType}
            bind:selectedValue={form.workType}
        />
    </div>

    <div>
        <Label for="seniorityLevel" class="py-2">Seniority Level</Label>
        <CustomEnumSelector
            enumObject={SeniorityLevel}
            bind:selectedValue={form.seniorityLevel}
        />
    </div>

    <div>
        <Label for="currency" class="py-2">Currency</Label>
        <CustomEnumSelector
            enumObject={Currency}
            bind:selectedValue={form.currency}
        />
    </div>

    <div>
        <Label for="salaryMin" class="py-2">Min Salary</Label>
        <Input
            id="salaryMin"
            type="number"
            bind:value={form.salaryMin}
            placeholder="50000"
        />
    </div>

    <div>
        <Label for="salaryMax" class="py-2">Max Salary</Label>
        <Input
            id="salaryMax"
            type="number"
            bind:value={form.salaryMax}
            placeholder="70000"
        />
    </div>

    <div>
        <Label for="description" class="py-2">Description</Label>
        <Textarea
            id="description"
            bind:value={form.description}
            placeholder="Job description..."
        />
    </div>

    <div>
        <Label for="url" class="py-2">Application URL</Label>
        <Input
            id="url"
            bind:value={form.url}
            placeholder="https://example.com/job"
        />
    </div>

    <div class="flex justify-between mt-4">
        <Button
            variant="destructive"
            href={caller ? `/${caller}/create?` : "/jobListings"}
        >
            Cancel
        </Button>
        <Button disabled={invalidSubmit} onclick={submit}>Submit</Button>
    </div>
</section>
