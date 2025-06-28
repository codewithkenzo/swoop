import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { Zap } from "lucide-react";

interface HeroProps {
  className?: string;
}

export default function Hero({ className }: HeroProps) {
  return (
    <section
      className={cn(
        "relative flex flex-col items-center justify-center text-center gap-6 py-24 md:py-40 px-4 md:px-6 overflow-hidden",
        className
      )}
    >
      {/* Gradient blobs */}
      <div className="absolute inset-0 -z-10 overflow-hidden">
        <div className="absolute -top-32 -left-32 h-96 w-96 rounded-full bg-violet-500 blur-[120px] opacity-50" />
        <div className="absolute bottom-0 right-0 h-72 w-72 rounded-full bg-cyan-400 blur-[100px] opacity-40" />
      </div>

      <motion.div
        initial={{ opacity: 0, y: 40 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true }}
        transition={{ duration: 0.6 }}
        className="inline-flex items-center gap-2 rounded-full bg-muted px-4 py-1 text-sm font-medium"
      >
        <Zap className="h-4 w-4 text-primary" />
        Real-time Document Intelligence
      </motion.div>

      <motion.h1
        initial={{ opacity: 0, y: 40 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true }}
        transition={{ duration: 0.7, delay: 0.1 }}
        className="text-4xl font-extrabold tracking-tight sm:text-5xl md:text-6xl lg:text-7xl bg-gradient-to-r from-violet-500 to-cyan-400 bg-clip-text text-transparent"
      >
        Understand &amp; Talk to Your Documents
      </motion.h1>

      <motion.p
        initial={{ opacity: 0, y: 40 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true }}
        transition={{ duration: 0.7, delay: 0.2 }}
        className="mx-auto max-w-2xl text-lg md:text-xl text-muted-foreground"
      >
        Swoop ingests, semantically indexes, &amp; streams answers (and audio) from thousands of files in
        milliseconds — powered by edge-deployed RAG &amp; neural TTS.
      </motion.p>

      <motion.div
        initial={{ opacity: 0, y: 40 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true }}
        transition={{ duration: 0.7, delay: 0.3 }}
        className="flex flex-col sm:flex-row gap-4"
      >
        <Button size="lg">Live Demo</Button>
        <Button variant="outline" size="lg">
          GitHub ↗
        </Button>
      </motion.div>
    </section>
  );
} 