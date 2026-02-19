import type { Metadata } from 'next';
import Image from 'next/image';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import { getProduct, formatPrice } from '@/lib/api';
import { PriceDisplay } from '@/components/PriceDisplay';
import { AddToCartButton } from './AddToCartButton';

interface ProductDetailPageProps {
  params: Promise<{ id: string }>;
}

export async function generateMetadata({
  params,
}: ProductDetailPageProps): Promise<Metadata> {
  const { id } = await params;
  try {
    const product = await getProduct(id);
    return {
      title: `${product.name} - Goseli`,
      description: product.short_description ?? product.description ?? product.name,
    };
  } catch {
    return {
      title: 'Product Not Found - Goseli',
      description: 'The product you are looking for could not be found.',
    };
  }
}

export default async function ProductDetailPage({ params }: ProductDetailPageProps) {
  const { id } = await params;

  let product;
  try {
    product = await getProduct(id);
  } catch {
    notFound();
  }

  const sortedImages = [...product.images].sort((a, b) => a.sort_order - b.sort_order);
  const primaryImage =
    sortedImages.find((img) => img.is_primary) ?? sortedImages[0] ?? null;
  const hasMultipleImages = sortedImages.length > 1;
  const activeVariants = product.variants.filter((v) => v.is_active);
  const inStock = product.stock_quantity > 0;

  return (
    <div className="container-custom py-8">
      <nav aria-label="Breadcrumb" className="mb-6">
        <ol className="flex items-center gap-2 text-sm text-neutral-600">
          <li>
            <Link href="/" className="hover:text-primary">
              Home
            </Link>
          </li>
          <li aria-hidden="true">/</li>
          <li>
            <Link href="/products" className="hover:text-primary">
              Products
            </Link>
          </li>
          <li aria-hidden="true">/</li>
          <li className="text-neutral-900 font-medium">{product.name}</li>
        </ol>
      </nav>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12">
        <div>
          <div className="relative aspect-square bg-neutral-100 rounded-lg overflow-hidden mb-4">
            {primaryImage ? (
              <Image
                src={primaryImage.url}
                alt={primaryImage.alt_text ?? product.name}
                fill
                sizes="(max-width: 1024px) 100vw, 50vw"
                priority
                className="object-cover"
              />
            ) : (
              <div className="flex h-full items-center justify-center bg-gradient-to-br from-neutral-100 to-neutral-200">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-24 w-24 text-neutral-400"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={1}
                    d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                  />
                </svg>
              </div>
            )}
          </div>

          {hasMultipleImages && (
            <div className="grid grid-cols-4 gap-2">
              {sortedImages.map((img) => (
                <div
                  key={img.id}
                  className="relative aspect-square bg-neutral-100 rounded overflow-hidden"
                >
                  <Image
                    src={img.url}
                    alt={img.alt_text ?? product.name}
                    fill
                    sizes="(max-width: 1024px) 25vw, 12.5vw"
                    className="object-cover"
                  />
                </div>
              ))}
            </div>
          )}
        </div>

        <div>
          {product.category && (
            <Link
              href={`/products?category_id=${product.category.id}`}
              className="badge badge-primary mb-3 inline-block"
            >
              {product.category.name}
            </Link>
          )}

          <h1 className="text-3xl font-bold text-neutral-900 mb-4">{product.name}</h1>

          <PriceDisplay
            price={product.price}
            compareAtPrice={product.compare_at_price}
            className="text-2xl mb-4"
          />

          <div className="mb-6">
            {inStock ? (
              <span className="badge badge-success">In Stock</span>
            ) : (
              <span className="badge badge-error">Out of Stock</span>
            )}
          </div>

          {product.short_description && (
            <p className="text-neutral-700 mb-6 leading-relaxed">
              {product.short_description}
            </p>
          )}

          {activeVariants.length > 0 && (
            <div className="mb-6">
              <h2 className="text-sm font-semibold text-neutral-700 mb-2">Variants</h2>
              <ul className="space-y-2">
                {activeVariants.map((variant) => (
                  <li key={variant.id} className="flex items-center justify-between">
                    <span className="text-neutral-900">{variant.name}</span>
                    <span className="badge badge-neutral">{formatPrice(variant.price)}</span>
                  </li>
                ))}
              </ul>
            </div>
          )}

          <AddToCartButton productId={product.id} inStock={inStock} />

          {product.sku && (
            <p className="text-sm text-neutral-500 mt-4">SKU: {product.sku}</p>
          )}
        </div>
      </div>

      {product.description && (
        <div className="mt-12 pt-8 border-t border-neutral-200">
          <h2 className="text-2xl font-bold text-neutral-900 mb-4">Description</h2>
          <div className="prose max-w-none text-neutral-700 leading-relaxed">
            {product.description}
          </div>
        </div>
      )}
    </div>
  );
}
