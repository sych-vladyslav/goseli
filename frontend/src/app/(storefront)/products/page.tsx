import type { Metadata } from 'next';
import { getCategories } from '@/lib/api';
import { ProductGrid } from './ProductGrid';
import type { Category } from '@/lib/types';

export const metadata: Metadata = {
  title: 'Products - Goseli',
  description: 'Browse our full product catalog.',
};

export default async function ProductsPage() {
  let categories: Category[] = [];
  try {
    categories = await getCategories();
  } catch (error) {
    categories = [];
  }

  return (
    <div className="container-custom py-8">
      <h1 className="text-3xl font-bold text-neutral-900 mb-6">Products</h1>
      <ProductGrid initialCategories={categories} />
    </div>
  );
}
