import { cx } from 'utils';
import ScoreStarSVG from 'assets/images/icons/ic-star-icon.svg';
import styles from './ScorePPV.module.scss';
import { ScorePPVProps } from './ScorePPV.interface';
import { useFTBalance } from '../../hooks';

function ScorePPV({ children }: ScorePPVProps) {
  const { balance, isFTBalanceReady } = useFTBalance();

  return (
    <>
      {isFTBalanceReady && (
        <div className={cx(styles['score-content'])}>
          <img src={ScoreStarSVG} alt="score star" className={cx(styles['score-content-img'])} />
          <span className={cx(styles['score-content-value'])}> {children || balance} </span>
          <span className={cx(styles['score-content-unit'])}>PPV</span>
        </div>
      )}
    </>
  );
}

export { ScorePPV };
