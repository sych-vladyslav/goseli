'use client';

import { useState, useCallback } from 'react';
import type { Category, ProductSort } from '@/lib/types';
import { useProducts } from '@/lib/hooks';
import { ProductCard } from '@/components/ProductCard';
import { Pagination } from '@/components/Pagination';
import { SearchBar } from '@/components/SearchBar';
import { CategoryFilter } from '@/components/CategoryFilter';
import { SortSelect } from '@/components/SortSelect';

interface ProductGridProps {
  initialCategories: Category[];
}

export function ProductGrid({ initialCategories }: ProductGridProps) {
  const [page, setPage] = useState(1);
  const [categoryId, setCategoryId] = useState<string | null>(null);
  const [sort, setSort] = useState<ProductSort>('created_at_desc');
  const [query, setQuery] = useState('');

  const { data, error, isLoading } = useProducts({
    page,
    per_page: 12,
    status: 'active',
    category_id: categoryId ?? undefined,
    sort,
    q: query || undefined,
  });

  const handleCategoryChange = useCallback((newCategoryId: string | null) => {
    setCategoryId(newCategoryId);
    setPage(1);
  }, []);

  const handleSortChange = useCallback((newSort: ProductSort) => {
    setSort(newSort);
    setPage(1);
  }, []);

  const handleSearch = useCallback((newQuery: string) => {
    setQuery(newQuery);
    setPage(1);
  }, []);

  const handlePageChange = useCallback((newPage: number) => {
    setPage(newPage);
  }, []);

  const handleClearFilters = useCallback(() => {
    setCategoryId(null);
    setQuery('');
    setSort('created_at_desc');
    setPage(1);
  }, []);

  const hasActiveFilters = categoryId !== null || query !== '' || sort !== 'created_at_desc';

  return (
    <div className="flex flex-col lg:flex-row gap-6">
      <aside className="lg:w-60 shrink-0">
        <CategoryFilter
          categories={initialCategories}
          selectedId={categoryId}
          onSelect={handleCategoryChange}
        />
      </aside>

      <div className="flex-1 min-w-0">
        <div className="flex flex-col sm:flex-row gap-4 mb-6">
          <div className="flex-1">
            <SearchBar
              initialQuery={query}
              onSearch={handleSearch}
              placeholder="Search products..."
            />
          </div>
          <SortSelect value={sort} onChange={handleSortChange} />
        </div>

        {isLoading && !data ? (
          <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6">
            {Array.from({ length: 6 }).map((_, i) => (
              <div key={i} className="card animate-pulse">
                <div className="relative aspect-square bg-neutral-200" />
                <div className="card-body">
                  <div className="h-4 bg-neutral-200 rounded w-3/4 mb-2" />
                  <div className="h-4 bg-neutral-200 rounded w-1/2" />
                </div>
              </div>
            ))}
          </div>
        ) : error ? (
          <div className="text-center py-12">
            <p className="text-error text-lg">Failed to load products. Please try again.</p>
          </div>
        ) : data && data.data.length === 0 ? (
          <div className="text-center py-12">
            <p className="text-neutral-600 text-lg mb-4">No products found.</p>
            {hasActiveFilters && (
              <button onClick={handleClearFilters} className="btn btn-outline btn-md">
                Clear filters
              </button>
            )}
          </div>
        ) : data ? (
          <>
            <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6">
              {data.data.map((product) => (
                <ProductCard key={product.id} product={product} />
              ))}
            </div>
            <Pagination pagination={data.pagination} onPageChange={handlePageChange} />
          </>
        ) : null}
      </div>
    </div>
  );
}
