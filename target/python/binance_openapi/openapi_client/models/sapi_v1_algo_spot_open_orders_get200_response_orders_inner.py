# coding: utf-8

"""
    Binance Public Spot API

    OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List
from typing import Optional, Set
from typing_extensions import Self

class SapiV1AlgoSpotOpenOrdersGet200ResponseOrdersInner(BaseModel):
    """
    SapiV1AlgoSpotOpenOrdersGet200ResponseOrdersInner
    """ # noqa: E501
    algo_id: StrictInt = Field(alias="algoId")
    symbol: StrictStr
    side: StrictStr
    total_qty: StrictStr = Field(alias="totalQty")
    executed_qty: StrictStr = Field(alias="executedQty")
    executed_amt: StrictStr = Field(alias="executedAmt")
    avg_price: StrictStr = Field(alias="avgPrice")
    client_algo_id: StrictStr = Field(alias="clientAlgoId")
    book_time: StrictInt = Field(alias="bookTime")
    end_time: StrictInt = Field(alias="endTime")
    algo_status: StrictStr = Field(alias="algoStatus")
    algo_type: StrictStr = Field(alias="algoType")
    urgency: StrictStr
    __properties: ClassVar[List[str]] = ["algoId", "symbol", "side", "totalQty", "executedQty", "executedAmt", "avgPrice", "clientAlgoId", "bookTime", "endTime", "algoStatus", "algoType", "urgency"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of SapiV1AlgoSpotOpenOrdersGet200ResponseOrdersInner from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SapiV1AlgoSpotOpenOrdersGet200ResponseOrdersInner from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "algoId": obj.get("algoId"),
            "symbol": obj.get("symbol"),
            "side": obj.get("side"),
            "totalQty": obj.get("totalQty"),
            "executedQty": obj.get("executedQty"),
            "executedAmt": obj.get("executedAmt"),
            "avgPrice": obj.get("avgPrice"),
            "clientAlgoId": obj.get("clientAlgoId"),
            "bookTime": obj.get("bookTime"),
            "endTime": obj.get("endTime"),
            "algoStatus": obj.get("algoStatus"),
            "algoType": obj.get("algoType"),
            "urgency": obj.get("urgency")
        })
        return _obj


