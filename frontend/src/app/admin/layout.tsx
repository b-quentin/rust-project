import * as React from "react"
import { ThemeProvider } from "@components/theme-provider"
import ModeToggle from "@components/mode-toggle";
import "@styles"

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <html lang="en" suppressHydrationWarning>
        <head />
        <body>
          <ThemeProvider
            attribute="class"
            defaultTheme="system"
            enableSystem
            disableTransitionOnChange
          >
            <header className="flex justify-end p-4">
              {/* Add the mode toggle button to your header */}
              <ModeToggle />
            </header>
            {children}
          </ThemeProvider>


        </body>
      </html>
    </>
  )
}

