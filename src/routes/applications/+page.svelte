<script lang="ts">
    import { applications, deleteApplication } from "$lib/stores/applications";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import { Stage } from "$lib/types/enums";
    import PencilIcon from "lucide-svelte/icons/pencil";
    import CheckSquareIcon from "lucide-svelte/icons/check-square";
    import CalendarIcon from "lucide-svelte/icons/calendar";
    import DeleteAlert from "$lib/components/forms/utils/DeleteAlert.svelte";

    let confirmDelete = $state(false);
    let selectedApplicationId: number | null = $state(null);
    let selectedApplicationLabel: string = $state("");

    $effect(() => {
        if (confirmDelete && selectedApplicationId) {
            handleDelete(selectedApplicationId);
            confirmDelete = false;
            selectedApplicationId = null;
            selectedApplicationLabel = "";
        }
    });

    async function handleDelete(id: number) {
        await deleteApplication(id);
    }

    // ----------------------------------------------------------
    // Helpers
    // ----------------------------------------------------------
    const stageColorMap: Record<Stage, string> = {
        [Stage.Applied]: "bg-blue-100 text-blue-800",
        [Stage.Screening]: "bg-purple-100 text-purple-800",
        [Stage.Assessment]: "bg-teal-100 text-teal-800",
        [Stage.Interviewing]: "bg-yellow-100 text-yellow-800",
        [Stage.Offered]: "bg-green-100 text-green-800",
        [Stage.Rejected]: "bg-red-100 text-red-800",
        [Stage.Withdrawn]: "bg-gray-200 text-gray-800",
        [Stage.OnHold]: "bg-orange-100 text-orange-800",
        [Stage.Other]: "bg-slate-100 text-slate-800",
        [Stage.Negotiation]: "bg-indigo-100 text-indigo-800",
        [Stage.Accepted]: "bg-emerald-100 text-emerald-800",
    };

    function stageColor(stage?: Stage): string {
        return stage ? stageColorMap[stage] : "bg-muted text-foreground";
    }

    function formatDate(dateStr: string): string {
        if (!dateStr) return "—";
        const date = new Date(dateStr);
        if (isNaN(date.getTime())) return dateStr;
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    // Count applications by stage
    const stageCounts = $derived.by(() => {
        const counts: Record<Stage, number> = {
            [Stage.Applied]: 0,
            [Stage.Screening]: 0,
            [Stage.Assessment]: 0,
            [Stage.Interviewing]: 0,
            [Stage.Offered]: 0,
            [Stage.Rejected]: 0,
            [Stage.Withdrawn]: 0,
            [Stage.OnHold]: 0,
            [Stage.Other]: 0,
            [Stage.Negotiation]: 0,
            [Stage.Accepted]: 0,
        };
        $applications.forEach((app) => {
            counts[app.stage]++;
        });
        return counts;
    });
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-8 flex items-center justify-between">
    <div>
        <h1 class="text-3xl font-bold tracking-tight">Applications</h1>
        <p class="text-muted-foreground mt-1">
            Track and manage your job applications
        </p>
    </div>
    <Button href="/applications/create" size="lg">+ New Application</Button>
</div>

<!-- ----------------------------------------------------------
Stats Cards
----------------------------------------------------------- -->
<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
    <div class="rounded-lg border border-border bg-background p-4">
        <div class="text-sm font-medium text-muted-foreground mb-1">
            Total Applications
        </div>
        <div class="text-3xl font-bold">{$applications.length}</div>
    </div>
    <div class="rounded-lg border border-border bg-background p-4">
        <div class="text-sm font-medium text-muted-foreground mb-1">
            In Progress
        </div>
        <div class="text-3xl font-bold text-blue-600">
            {stageCounts[Stage.Applied] +
                stageCounts[Stage.Screening] +
                stageCounts[Stage.Assessment] +
                stageCounts[Stage.Interviewing]}
        </div>
    </div>
    <div class="rounded-lg border border-border bg-background p-4">
        <div class="text-sm font-medium text-muted-foreground mb-1">
            Offered
        </div>
        <div class="text-3xl font-bold text-green-600">
            {stageCounts[Stage.Offered]}
        </div>
    </div>
    <div class="rounded-lg border border-border bg-background p-4">
        <div class="text-sm font-medium text-muted-foreground mb-1">
            Rejected
        </div>
        <div class="text-3xl font-bold text-red-600">
            {stageCounts[Stage.Rejected]}
        </div>
    </div>
</div>

<!-- ----------------------------------------------------------
Empty State
----------------------------------------------------------- -->
{#if $applications.length === 0}
    <div
        class="rounded-lg border border-dashed border-border bg-muted/30 p-12 text-center"
    >
        <CheckSquareIcon
            class="mx-auto mb-4 size-12 text-muted-foreground opacity-50"
        />
        <h3 class="font-semibold text-foreground mb-1">No applications yet</h3>
        <p class="text-sm text-muted-foreground mb-4">
            Start tracking your job applications
        </p>
        <Button href="/applications/create">Create First Application</Button>
    </div>
{:else}
    <!-- ----------------------------------------------------------
    Applications Table
    ----------------------------------------------------------- -->
    <div
        class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
    >
        <table class="w-full text-sm">
            <thead
                class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground border-b border-border"
            >
                <tr>
                    <th class="px-6 py-4 font-semibold">Job Listing</th>
                    <th class="px-6 py-4 font-semibold">Stage</th>
                    <th class="px-6 py-4 font-semibold">Applied</th>
                    <th class="px-6 py-4 font-semibold">Notes</th>
                    <th class="px-6 py-4 text-right font-semibold">Actions</th>
                </tr>
            </thead>

            <tbody>
                {#each $applications as application (application.id)}
                    <tr
                        class="border-t border-border hover:bg-muted/50 transition-colors"
                    >
                        <!-- Job Listing -->
                        <td class="px-6 py-4">
                            <div class="font-semibold text-foreground">
                                {application.displayLabel ??
                                    application.jobListingId ??
                                    "—"}
                            </div>
                        </td>

                        <!-- Stage -->
                        <td class="px-6 py-4">
                            <Badge class={stageColor(application.stage)}>
                                {application.stage}
                            </Badge>
                        </td>

                        <!-- Applied Date -->
                        <td class="px-6 py-4">
                            <div
                                class="flex items-center gap-2 text-foreground"
                            >
                                <CalendarIcon
                                    class="size-4 text-muted-foreground"
                                />
                                {formatDate(application.appliedDate)}
                            </div>
                        </td>

                        <!-- Notes -->
                        <td
                            class="px-6 py-4 truncate max-w-[220px] text-muted-foreground"
                        >
                            {application.applicationNotes || "—"}
                        </td>

                        <!-- Actions -->
                        <td class="px-6 py-4">
                            <div class="flex justify-end gap-2">
                                <Button
                                    size="sm"
                                    variant="outline"
                                    class="gap-2"
                                    href="/applications/{application.id}"
                                >
                                    <PencilIcon class="size-4" />
                                    Edit
                                </Button>
                                <DeleteAlert
                                    objectText="'{application.displayLabel ??
                                        application.jobListingId ??
                                        'Application'}'"
                                    description="This will delete the application record."
                                    onDelete={async () => {
                                        await deleteApplication(
                                            application.id!,
                                        );
                                    }}
                                />
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
