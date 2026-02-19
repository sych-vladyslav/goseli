import Link from 'next/link';

export default function ProductNotFound() {
  return (
    <div className="container-custom py-16 text-center">
      <h1 className="text-4xl font-bold text-neutral-900 mb-4">Product Not Found</h1>
      <p className="text-neutral-600 text-lg mb-8">
        The product you are looking for could not be found or may have been removed.
      </p>
      <Link href="/products" className="btn btn-primary btn-md">
        Browse All Products
      </Link>
    </div>
  );
}
