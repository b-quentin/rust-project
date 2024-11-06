import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "@components/app-sidebar";
import ModeToggle from "@components/mode-toggle";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <SidebarProvider>
      <AppSidebar />
      <main className="w-full">
        <header className="flex items-center justify-between w-full p-4">
          {/* Sidebar trigger on the left */}
          <SidebarTrigger />
          {/* Mode toggle button on the right */}
          <ModeToggle />
        </header>
        {children}
      </main>
    </SidebarProvider>
  );
}
