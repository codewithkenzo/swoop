import { Button } from "@/components/ui/button";
import { motion } from "framer-motion";

export default function CTA() {
  return (
    <section className="py-20 lg:py-32">
      <motion.div
        className="container bg-muted rounded-2xl p-10 flex flex-col items-center gap-6 text-center"
        initial={{ opacity: 0, y: 40 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true }}
        transition={{ duration: 0.6 }}
      >
        <h2 className="text-3xl md:text-4xl font-bold tracking-tight">
          Ready to unlock your documents?
        </h2>
        <p className="max-w-xl text-muted-foreground">
          Swoop turns static files into dynamic knowledge. Start your free trial — no credit card required.
        </p>
        <div className="flex flex-col sm:flex-row gap-4">
          <Button size="lg">Start Free</Button>
          <Button variant="outline" size="lg">
            Book Demo
          </Button>
        </div>
      </motion.div>
    </section>
  );
} 