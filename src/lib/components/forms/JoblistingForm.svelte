<script lang="ts">
    import { goto } from "$app/navigation";
    import CustomIDSelectCreate from "$lib/components/formDialogs/utils/CustomIDSelectCreate.svelte";
    import { Button } from "$lib/components/ui/button";
    import { companies, createCompany } from "$lib/stores/companies";
    import {
        createJobListing,
        jobListings,
        updateJobListing,
    } from "$lib/stores/jobListings";
    import {
        Currency,
        CurrencyDisplay,
        SeniorityLevel,
        SeniorityLevelDisplay,
        WorkType,
        WorkTypeDisplay,
    } from "$lib/types/enums";
    import type { JobListing } from "$lib/types/jobListing";
    import CustomEnumSelector from "../formDialogs/utils/CustomEnumSelector.svelte";
    import { Input } from "../ui/input";
    import { Label } from "../ui/label";
    import { Textarea } from "../ui/textarea";

    let { joblistingID }: { joblistingID?: number } = $props();

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
    $effect(() => {
        if (!joblistingID) return;
        const found = $jobListings.find((j) => j.id === joblistingID);
        if (found) {
            Object.assign(form, found);
            // Also sync the company selector
            if (found.companyId) {
                result = [found.companyId, false];
            }
        }
    });

    // State is a tuple: [Value (ID or String), IsManualMode]
    let result = $state<[number | string | undefined, boolean]>([
        undefined,
        false,
    ]);

    const invalidSubmit = $derived(
        () => result[0] === undefined || result[0] === "",
    );

    async function submit() {
        if (invalidSubmit()) return;

        const [companyValue, isManual] = result;

        // If manual mode, create a new company first
        if (isManual) {
            const newCompany = await createCompany({
                name: String(companyValue),
            });
            form.companyId = newCompany.id;
        } else {
            form.companyId = Number(companyValue);
        }

        // Create or update job listing
        if (joblistingID) {
            await updateJobListing(Number(joblistingID), form);
        } else {
            await createJobListing(form);
        }

        console.log($state.snapshot(form));
        goto("/jobListings");
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-3">
    <h1 class="text-xl font-extrabold py-4">Create a new Job Listing</h1>

    <div>
        <Label class="py-2 required">Company:</Label>
        <CustomIDSelectCreate {items} bind:value={result} />
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
        <Button variant="destructive" href="/jobListings">Cancel</Button>
        <Button disabled={invalidSubmit()} onclick={submit}>Submit</Button>
    </div>
</section>
