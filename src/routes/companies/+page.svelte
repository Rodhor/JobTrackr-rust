<script lang="ts">
    import { companies, deleteCompany } from "$lib/stores/companies";
    import { jobListings } from "$lib/stores/jobListings";
    import { Button } from "$lib/components/ui/button";
    import Trash2Icon from "lucide-svelte/icons/trash-2";
    import PencilIcon from "lucide-svelte/icons/pencil";
    import BuildingIcon from "lucide-svelte/icons/building-2";
    import CalendarIcon from "lucide-svelte/icons/calendar";
    import LinkIcon from "lucide-svelte/icons/link";

    async function handleDelete(id: number) {
        await deleteCompany(id);
        // Manually remove job listings that belonged to the deleted company
        jobListings.update((listings) =>
            listings.filter((listing) => listing.companyId !== id),
        );
    }

    function formatDate(dateStr: string) {
        if (!dateStr) return "—";
        const date = new Date(dateStr);
        if (isNaN(date.getTime())) return dateStr;
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-8 flex items-center justify-between">
    <div>
        <h1 class="text-3xl font-bold tracking-tight">Companies</h1>
        <p class="text-muted-foreground mt-1">Manage your company database</p>
    </div>
    <Button href="/companies/create" size="lg">+ New Company</Button>
</div>

<!-- ----------------------------------------------------------
Stats Card
----------------------------------------------------------- -->
<div class="mb-8 rounded-lg border border-border bg-background p-4">
    <div class="flex items-center justify-between">
        <div>
            <div class="text-sm font-medium text-muted-foreground mb-1">
                Total Companies
            </div>
            <div class="text-3xl font-bold">{$companies.length}</div>
        </div>
        <BuildingIcon class="size-12 text-muted-foreground opacity-30" />
    </div>
</div>

<!-- ----------------------------------------------------------
Empty State
----------------------------------------------------------- -->
{#if $companies.length === 0}
    <div
        class="rounded-lg border border-dashed border-border bg-muted/30 p-12 text-center"
    >
        <BuildingIcon
            class="mx-auto mb-4 size-12 text-muted-foreground opacity-50"
        />
        <h3 class="font-semibold text-foreground mb-1">No companies yet</h3>
        <p class="text-sm text-muted-foreground mb-4">
            Start building your company database
        </p>
        <Button href="/companies/create">Add First Company</Button>
    </div>
{:else}
    <!-- ----------------------------------------------------------
    Companies Table
    ----------------------------------------------------------- -->
    <div
        class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
    >
        <table class="w-full text-sm">
            <thead
                class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground border-b border-border"
            >
                <tr>
                    <th class="px-6 py-4 font-semibold">Name</th>
                    <th class="px-6 py-4 font-semibold">City</th>
                    <th class="px-6 py-4 font-semibold">Country</th>
                    <th class="px-6 py-4 font-semibold">Industry</th>
                    <th class="px-6 py-4 font-semibold">Website</th>
                    <th class="px-6 py-4 font-semibold">Last Updated</th>
                    <th class="px-6 py-4 text-right font-semibold">Actions</th>
                </tr>
            </thead>
            <tbody>
                {#each $companies as company (company.id)}
                    <tr
                        class="border-t border-border hover:bg-muted/50 transition-colors"
                    >
                        <!-- Name -->
                        <td class="px-6 py-4">
                            <div class="font-semibold text-foreground">
                                {company.name}
                            </div>
                        </td>

                        <!-- City -->
                        <td class="px-6 py-4 text-muted-foreground">
                            {company.city || "—"}
                        </td>

                        <!-- Country -->
                        <td class="px-6 py-4 text-muted-foreground">
                            {company.country || "—"}
                        </td>

                        <!-- Industry -->
                        <td
                            class="px-6 py-4 text-muted-foreground truncate max-w-[160px]"
                        >
                            {company.industry || "—"}
                        </td>

                        <!-- Website -->
                        <td class="px-6 py-4">
                            {#if company.website}
                                <a
                                    href={company.website}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center gap-2 text-blue-600 hover:underline truncate max-w-[200px]"
                                    title={company.website}
                                >
                                    <LinkIcon class="size-4 flex-shrink-0" />
                                    <span class="truncate">Website</span>
                                </a>
                            {:else}
                                <span class="text-muted-foreground">—</span>
                            {/if}
                        </td>

                        <!-- Last Updated -->
                        <td class="px-6 py-4">
                            <div
                                class="flex items-center gap-2 text-foreground"
                            >
                                <CalendarIcon
                                    class="size-4 text-muted-foreground"
                                />
                                {company.updatedAt
                                    ? formatDate(company.updatedAt)
                                    : "—"}
                            </div>
                        </td>

                        <!-- Actions -->
                        <td class="px-6 py-4">
                            <div class="flex justify-end gap-2">
                                <Button
                                    size="sm"
                                    variant="outline"
                                    class="gap-2"
                                    href="/companies/{company.id}"
                                >
                                    <PencilIcon class="size-4" />
                                    Edit
                                </Button>
                                <Button
                                    size="sm"
                                    variant="destructive"
                                    class="gap-2"
                                    onclick={() =>
                                        company.id && handleDelete(company.id)}
                                >
                                    <Trash2Icon class="size-4" />
                                    Delete
                                </Button>
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
