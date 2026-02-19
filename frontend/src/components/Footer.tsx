export function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="border-t border-neutral-200 bg-neutral-50 mt-auto">
      <div className="container-custom py-8">
        <div className="flex flex-col sm:flex-row items-center justify-between gap-4">
          <p className="text-sm text-neutral-500">
            &copy; {currentYear} Goseli. All rights reserved.
          </p>
          <nav aria-label="Footer navigation" className="flex gap-4">
            <a href="#" className="text-sm text-neutral-500 hover:text-primary">
              Privacy
            </a>
            <a href="#" className="text-sm text-neutral-500 hover:text-primary">
              Terms
            </a>
          </nav>
        </div>
      </div>
    </footer>
  );
}
