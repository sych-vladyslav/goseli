import Link from 'next/link'
import { Metadata } from 'next'
import { getProducts } from '@/lib/api'
import { ProductCard } from '@/components/ProductCard'
import type { Product } from '@/lib/types'

export const metadata: Metadata = {
  title: 'Goseli - Your Online Store',
  description: 'Discover our curated selection of products.',
}

export default async function HomePage() {
  let products: Product[] = []
  try {
    const response = await getProducts({
      status: 'active',
      per_page: 8,
      sort: 'created_at_desc'
    })
    products = response.data
  } catch (error) {
    products = []
  }

  return (
    <>
      {/* Hero Section */}
      <section className="bg-gradient-to-r from-primary to-secondary text-white py-20">
        <div className="container-custom">
          <h1 className="text-5xl font-bold mb-4">Welcome to Our Store</h1>
          <p className="text-xl mb-8 opacity-90">
            Discover our curated selection of products.
          </p>
          <Link href="/products" className="btn btn-accent btn-lg">
            Shop Now
          </Link>
        </div>
      </section>

      {/* Featured Products Section */}
      <section className="py-16">
        <div className="container-custom">
          <h2 className="text-3xl font-bold text-neutral-900 mb-8">
            Featured Products
          </h2>

          {products.length === 0 ? (
            <p className="text-neutral-600 text-center py-12">
              No products available yet. Check back soon!
            </p>
          ) : (
            <>
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                {products.map((product) => (
                  <ProductCard key={product.id} product={product} />
                ))}
              </div>
              <div className="text-center">
                <Link
                  href="/products"
                  className="text-primary hover:text-primary-dark font-medium inline-flex items-center gap-2"
                >
                  View all →
                </Link>
              </div>
            </>
          )}
        </div>
      </section>
    </>
  )
}
