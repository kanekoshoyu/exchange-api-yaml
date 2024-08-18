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

from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictStr
from typing import Any, ClassVar, Dict, List
from typing import Optional, Set
from typing_extensions import Self

class SapiV1SimpleEarnLockedPositionGet200ResponseRowsInner(BaseModel):
    """
    SapiV1SimpleEarnLockedPositionGet200ResponseRowsInner
    """ # noqa: E501
    position_id: StrictStr = Field(alias="positionId")
    project_id: StrictStr = Field(alias="projectId")
    asset: StrictStr
    amount: StrictStr
    purchase_time: StrictStr = Field(alias="purchaseTime")
    duration: StrictStr
    accrual_days: StrictStr = Field(alias="accrualDays")
    reward_asset: StrictStr = Field(alias="rewardAsset")
    apy: StrictStr = Field(alias="APY")
    is_renewable: StrictBool = Field(alias="isRenewable")
    is_auto_renew: StrictBool = Field(alias="isAutoRenew")
    redeem_date: StrictStr = Field(alias="redeemDate")
    __properties: ClassVar[List[str]] = ["positionId", "projectId", "asset", "amount", "purchaseTime", "duration", "accrualDays", "rewardAsset", "APY", "isRenewable", "isAutoRenew", "redeemDate"]

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
        """Create an instance of SapiV1SimpleEarnLockedPositionGet200ResponseRowsInner from a JSON string"""
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
        """Create an instance of SapiV1SimpleEarnLockedPositionGet200ResponseRowsInner from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "positionId": obj.get("positionId"),
            "projectId": obj.get("projectId"),
            "asset": obj.get("asset"),
            "amount": obj.get("amount"),
            "purchaseTime": obj.get("purchaseTime"),
            "duration": obj.get("duration"),
            "accrualDays": obj.get("accrualDays"),
            "rewardAsset": obj.get("rewardAsset"),
            "APY": obj.get("APY"),
            "isRenewable": obj.get("isRenewable"),
            "isAutoRenew": obj.get("isAutoRenew"),
            "redeemDate": obj.get("redeemDate")
        })
        return _obj


