'use client';

import type { PaginationMeta } from '@/lib/types';

interface PaginationProps {
  pagination: PaginationMeta;
  onPageChange: (page: number) => void;
}

export function Pagination({ pagination, onPageChange }: PaginationProps) {
  const { page, total_pages } = pagination;
  if (total_pages <= 1) return null;

  const pages: number[] = [];
  const start = Math.max(1, page - 2);
  const end = Math.min(total_pages, page + 2);
  for (let i = start; i <= end; i++) {
    pages.push(i);
  }

  return (
    <nav aria-label="Pagination" className="flex items-center justify-center gap-1 mt-8">
      <button
        onClick={() => onPageChange(page - 1)}
        disabled={page <= 1}
        className="btn btn-ghost btn-sm"
        aria-label="Previous page"
      >
        Previous
      </button>

      {start > 1 && (
        <>
          <button onClick={() => onPageChange(1)} className="btn btn-ghost btn-sm" aria-label="Page 1">
            1
          </button>
          {start > 2 && <span className="px-2 text-neutral-400" aria-hidden="true">...</span>}
        </>
      )}

      {pages.map((p) => (
        <button
          key={p}
          onClick={() => onPageChange(p)}
          className={`btn btn-sm ${p === page ? 'btn-primary' : 'btn-ghost'}`}
          aria-current={p === page ? 'page' : undefined}
          aria-label={`Page ${p}`}
        >
          {p}
        </button>
      ))}

      {end < total_pages && (
        <>
          {end < total_pages - 1 && <span className="px-2 text-neutral-400" aria-hidden="true">...</span>}
          <button
            onClick={() => onPageChange(total_pages)}
            className="btn btn-ghost btn-sm"
            aria-label={`Page ${total_pages}`}
          >
            {total_pages}
          </button>
        </>
      )}

      <button
        onClick={() => onPageChange(page + 1)}
        disabled={page >= total_pages}
        className="btn btn-ghost btn-sm"
        aria-label="Next page"
      >
        Next
      </button>
    </nav>
  );
}
