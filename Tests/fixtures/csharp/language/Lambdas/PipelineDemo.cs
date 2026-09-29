using Fx.Traps;

namespace Fx.Lambdas
{
    public class PipelineDemo
    {
        public int Build()
        {
            var pipeline = new Pipeline()
                .Then(MathSteps.Double)
                .Then(x => MathSteps.Increment(x))
                .Then(x => x * 10);
            return pipeline.Run(1);
        }

        public Money Bonus(Money salary)
        {
            // Trap: Money.Double() has the same name as MathSteps.Double.
            return salary.Double();
        }
    }
}
