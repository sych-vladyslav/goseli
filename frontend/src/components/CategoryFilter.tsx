'use client';

import type { Category } from '@/lib/types';

interface CategoryFilterProps {
  categories: Category[];
  selectedId: string | null;
  onSelect: (categoryId: string | null) => void;
}

export function CategoryFilter({ categories, selectedId, onSelect }: CategoryFilterProps) {
  if (categories.length === 0) return null;

  return (
    <div>
      <h3 className="text-sm font-semibold text-neutral-700 mb-2">Categories</h3>
      <ul className="space-y-1" role="listbox" aria-label="Filter by category">
        <li>
          <button
            className={`w-full text-left px-3 py-1.5 rounded-md text-sm transition-colors ${
              selectedId === null
                ? 'bg-primary text-white'
                : 'text-neutral-600 hover:bg-neutral-100'
            }`}
            onClick={() => onSelect(null)}
            role="option"
            aria-selected={selectedId === null}
          >
            All Categories
          </button>
        </li>
        {categories.map((cat) => (
          <li key={cat.id}>
            <button
              className={`w-full text-left px-3 py-1.5 rounded-md text-sm transition-colors ${
                selectedId === cat.id
                  ? 'bg-primary text-white'
                  : 'text-neutral-600 hover:bg-neutral-100'
              }`}
              onClick={() => onSelect(cat.id)}
              role="option"
              aria-selected={selectedId === cat.id}
            >
              {cat.name}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}
