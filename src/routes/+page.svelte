<script lang="ts">
    import { base } from "$app/paths";
    import {
        Card,
        CardHeader,
        CardTitle,
        CardContent,
        CardDescription,
    } from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import {
        Briefcase,
        Building2,
        StickyNote,
        Plus,
        ArrowRight,
    } from "lucide-svelte";

    // ---------------------------------------------------------------------
    // Mock Data — replace with Wails-bound data layer later
    // ---------------------------------------------------------------------
    const applications = [
        {
            id: 1,
            company: "Acme Corp",
            position: "Frontend Developer",
            status: "Interview",
        },
        {
            id: 2,
            company: "Globex",
            position: "Backend Engineer",
            status: "Applied",
        },
        {
            id: 3,
            company: "Initech",
            position: "Full-Stack Dev",
            status: "Rejected",
        },
    ];

    const companies = [
        {
            id: 1,
            name: "Acme Corp",
            industry: "Technology",
            location: "Berlin",
        },
        { id: 2, name: "Globex", industry: "Finance", location: "Hamburg" },
        { id: 3, name: "Initech", industry: "Consulting", location: "Munich" },
    ];

    const notes = [
        { id: 1, title: "Email recruiter at Acme", createdAt: "2025-10-20" },
        {
            id: 2,
            title: "Prepare interview questions",
            createdAt: "2025-10-21",
        },
        {
            id: 3,
            title: "Update cover letter for Globex",
            createdAt: "2025-10-22",
        },
    ];
</script>

<!-- ===================================================================== -->
<!-- Layout -->
<!-- ===================================================================== -->
<section class="space-y-10">
    <!-- ----------------------------------------------------------------- -->
    <!-- Overview Section -->
    <!-- ----------------------------------------------------------------- -->
    <div>
        <h1 class="text-3xl font-semibold tracking-tight mb-2">Welcome back</h1>
        <p class="text-sm text-muted-foreground mb-6">
            Here’s a quick look at your current job search activity.
        </p>

        <div class="grid gap-6 md:grid-cols-3">
            <!-- =========================================================== -->
            <!-- Applications -->
            <!-- =========================================================== -->
            <Card class="hover:bg-muted/10 transition-colors">
                <CardHeader class="pb-3">
                    <CardTitle class="flex items-center gap-2">
                        <Briefcase class="h-5 w-5 text-primary" />
                        Applications
                    </CardTitle>
                    <CardDescription
                        >Recent activity and statuses</CardDescription
                    >
                </CardHeader>
                <CardContent class="space-y-3">
                    {#each applications.slice(0, 3) as app}
                        <a
                            href={`${base}/applications/${app.id}`}
                            class="flex justify-between items-center rounded-md px-3 py-2 hover:bg-accent/20 transition-colors"
                        >
                            <div>
                                <p class="font-medium leading-tight">
                                    {app.position}
                                </p>
                                <p class="text-sm text-muted-foreground">
                                    {app.company}
                                </p>
                            </div>
                            <span class="text-xs text-muted-foreground"
                                >{app.status}</span
                            >
                        </a>
                        {#if app !== applications.slice(0, 3)[2]}
                            <Separator />
                        {/if}
                    {/each}

                    <div class="flex justify-between mt-4">
                        <a href={`${base}/applications/create`}>
                            <Button variant="secondary" size="sm">
                                <Plus class="h-4 w-4 mr-1" /> New
                            </Button>
                        </a>
                        <a href={`${base}/applications`}>
                            <Button
                                variant="ghost"
                                size="sm"
                                class="flex items-center gap-1"
                            >
                                View All <ArrowRight class="h-3 w-3" />
                            </Button>
                        </a>
                    </div>
                </CardContent>
            </Card>

            <!-- =========================================================== -->
            <!-- Companies -->
            <!-- =========================================================== -->
            <Card class="hover:bg-muted/10 transition-colors">
                <CardHeader class="pb-3">
                    <CardTitle class="flex items-center gap-2">
                        <Building2 class="h-5 w-5 text-primary" />
                        Companies
                    </CardTitle>
                    <CardDescription
                        >Your stored company profiles</CardDescription
                    >
                </CardHeader>
                <CardContent class="space-y-3">
                    {#each companies.slice(0, 3) as c}
                        <a
                            href={`${base}/companies/${c.id}`}
                            class="block rounded-md px-3 py-2 hover:bg-accent/20 transition-colors"
                        >
                            <p class="font-medium leading-tight">{c.name}</p>
                            <p class="text-sm text-muted-foreground">
                                {c.industry} · {c.location}
                            </p>
                        </a>
                        {#if c !== companies.slice(0, 3)[2]}
                            <Separator />
                        {/if}
                    {/each}

                    <div class="flex justify-between mt-4">
                        <a href={`${base}/companies/create`}>
                            <Button variant="secondary" size="sm">
                                <Plus class="h-4 w-4 mr-1" /> New
                            </Button>
                        </a>
                        <a href={`${base}/companies`}>
                            <Button
                                variant="ghost"
                                size="sm"
                                class="flex items-center gap-1"
                            >
                                View All <ArrowRight class="h-3 w-3" />
                            </Button>
                        </a>
                    </div>
                </CardContent>
            </Card>

            <!-- =========================================================== -->
            <!-- Notes -->
            <!-- =========================================================== -->
            <Card class="hover:bg-muted/10 transition-colors">
                <CardHeader class="pb-3">
                    <CardTitle class="flex items-center gap-2">
                        <StickyNote class="h-5 w-5 text-primary" />
                        Notes
                    </CardTitle>
                    <CardDescription
                        >Recent reminders or thoughts</CardDescription
                    >
                </CardHeader>
                <CardContent class="space-y-3">
                    {#each notes.slice(0, 3) as note}
                        <a
                            href={`${base}/notes/${note.id}`}
                            class="flex justify-between items-center rounded-md px-3 py-2 hover:bg-accent/20 transition-colors"
                        >
                            <p class="font-medium truncate">{note.title}</p>
                            <span class="text-xs text-muted-foreground"
                                >{note.createdAt}</span
                            >
                        </a>
                        {#if note !== notes.slice(0, 3)[2]}
                            <Separator />
                        {/if}
                    {/each}

                    <div class="flex justify-between mt-4">
                        <a href={`${base}/notes/create`}>
                            <Button variant="secondary" size="sm">
                                <Plus class="h-4 w-4 mr-1" /> New
                            </Button>
                        </a>
                        <a href={`${base}/notes`}>
                            <Button
                                variant="ghost"
                                size="sm"
                                class="flex items-center gap-1"
                            >
                                View All <ArrowRight class="h-3 w-3" />
                            </Button>
                        </a>
                    </div>
                </CardContent>
            </Card>
        </div>
    </div>

    <!-- ----------------------------------------------------------------- -->
    <!-- Quick Actions (optional secondary row) -->
    <!-- ----------------------------------------------------------------- -->
    <div>
        <h2 class="text-xl font-semibold mb-4">Quick Actions</h2>
        <div class="grid gap-4 sm:grid-cols-3">
            <a
                href={`${base}/applications/create`}
                class="flex items-center justify-center gap-2 rounded-lg border border-border bg-muted/30 hover:bg-accent/20 py-4 transition-colors"
            >
                <Briefcase class="h-5 w-5 text-primary" />
                <span class="text-sm font-medium">New Application</span>
            </a>
            <a
                href={`${base}/companies/create`}
                class="flex items-center justify-center gap-2 rounded-lg border border-border bg-muted/30 hover:bg-accent/20 py-4 transition-colors"
            >
                <Building2 class="h-5 w-5 text-primary" />
                <span class="text-sm font-medium">New Company</span>
            </a>
            <a
                href={`${base}/notes/create`}
                class="flex items-center justify-center gap-2 rounded-lg border border-border bg-muted/30 hover:bg-accent/20 py-4 transition-colors"
            >
                <StickyNote class="h-5 w-5 text-primary" />
                <span class="text-sm font-medium">New Note</span>
            </a>
        </div>
    </div>
</section>
