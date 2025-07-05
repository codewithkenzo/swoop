import Hero from "@/components/marketing/Hero";
import FeatureCards from "@/components/marketing/FeatureCards";
import CTA from "@/components/marketing/CTA";

export default function Landing() {
  return (
    <div className="min-h-screen flex flex-col bg-background font-sans">
      <Hero />
      <FeatureCards />
      <CTA />
    </div>
  );
} 