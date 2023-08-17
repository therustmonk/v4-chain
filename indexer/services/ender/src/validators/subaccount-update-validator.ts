import { IndexerTendermintEvent, SubaccountUpdateEventV1 } from '@dydxprotocol-indexer/v4-protos';

import { Handler } from '../handlers/handler';
import { SubaccountUpdateHandler } from '../handlers/subaccount-update-handler';
import { Validator } from './validator';

export class SubaccountUpdateValidator extends Validator<SubaccountUpdateEventV1> {
  public validate(): void {
    if (this.event.subaccountId === undefined) {
      this.logAndThrowParseMessageError(
        'SubaccountUpdateEvent must contain a subaccountId',
        { event: this.event },
      );
    }
  }

  public createHandlers(
    indexerTendermintEvent: IndexerTendermintEvent,
    txId: number,
  ): Handler<SubaccountUpdateEventV1>[] {
    return [
      new SubaccountUpdateHandler(
        this.block,
        indexerTendermintEvent,
        txId,
        this.event,
      ),
    ];
  }
}