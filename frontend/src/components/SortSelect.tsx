'use client';

import type { ProductSort } from '@/lib/types';

interface SortSelectProps {
  value: ProductSort | undefined;
  onChange: (sort: ProductSort) => void;
}

const SORT_OPTIONS: { value: ProductSort; label: string }[] = [
  { value: 'created_at_desc', label: 'Newest' },
  { value: 'name_asc', label: 'Name (A-Z)' },
  { value: 'price_asc', label: 'Price: Low to High' },
  { value: 'price_desc', label: 'Price: High to Low' },
];

export function SortSelect({ value, onChange }: SortSelectProps) {
  return (
    <div className="flex items-center gap-2">
      <label htmlFor="sort-select" className="text-sm text-neutral-600 whitespace-nowrap">
        Sort by
      </label>
      <select
        id="sort-select"
        className="select text-sm py-1.5"
        value={value ?? 'created_at_desc'}
        onChange={(e) => onChange(e.target.value as ProductSort)}
        aria-label="Sort products"
      >
        {SORT_OPTIONS.map((opt) => (
          <option key={opt.value} value={opt.value}>
            {opt.label}
          </option>
        ))}
      </select>
    </div>
  );
}
